use crate::gobject::{CallstackObject, DisassembledInstruction, MemoryDumpObject};
use crate::{BgEvent, Callstack, Disassembly, Event, Memory, Registers};
use async_channel::Sender;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use rustcube::cpu;

pub struct App {
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
    disassembly_column_view: gtk::ColumnView,
    disassembly_list_store: gtk::gio::ListStore,
    register_store: gtk::ListStore,
    registers: Registers,
    callstack_list_store: gtk::gio::ListStore,
    memory_list_store: gtk::gio::ListStore,
}

impl App {
    pub fn new(app: &gtk::Application, tx: Sender<Event>, btx: Sender<BgEvent>) -> Self {
        let ui_src = include_str!("app.ui");
        let builder = gtk::Builder::new();
        let scope = gtk::BuilderRustScope::new();

        builder.set_scope(Some(&scope));
        builder
            .add_from_string(ui_src)
            .expect("Couldn't add from string");

        let button_step: gtk::Button = builder
            .object("button_step")
            .expect("Couldn't get button_step");

        let button_continue: gtk::Button = builder
            .object("button_continue")
            .expect("Couldn't get button_continue");

        let button_stop: gtk::Button = builder
            .object("button_stop")
            .expect("Couldn't get button_stop");

        button_step.connect_clicked(clone!(@strong btx => move |_| {
            let btx = btx.clone();
            glib::MainContext::default().spawn_local(async move {
                let _ = btx.send(BgEvent::Step).await;
            });
        }));

        button_continue.connect_clicked(clone!(@strong btx => move |_| {
            let btx = btx.clone();
            glib::MainContext::default().spawn_local(async move {
                let _ = btx.send(BgEvent::Continue).await;
            });
        }));

        button_stop.connect_clicked(clone!(@strong btx => move |_| {
            let btx = btx.clone();
            glib::MainContext::default().spawn_local(async move {
                let _ = btx.send(BgEvent::Stop).await;
            });
        }));

        let disassembly_list_store =
            gtk::gio::ListStore::new(DisassembledInstruction::static_type());

        let sel = gtk::NoSelection::new(Some(&disassembly_list_store));
        let disassembly_column_view: gtk::ColumnView = builder
            .object("disassembly_column_view")
            .expect("disassembly_column_view");

        disassembly_column_view.set_model(Some(&sel));

        let callstack_list_store = gtk::gio::ListStore::new(CallstackObject::static_type());

        let sel = gtk::NoSelection::new(Some(&callstack_list_store));
        let callstack_list_view: gtk::ListView = builder
            .object("callstack_list_view")
            .expect("Couldn't get callstack_list_view");
        callstack_list_view.set_model(Some(&sel));

        let memory_list_store = gtk::gio::ListStore::new(MemoryDumpObject::static_type());

        let sel = gtk::NoSelection::new(Some(&memory_list_store));
        let memory_column_view: gtk::ColumnView = builder
            .object("memory_column_view")
            .expect("Couldn't get memory_column_view");
        memory_column_view.set_model(Some(&sel));

        let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");

        window.set_application(Some(app));

        let register_store: gtk::ListStore = builder
            .object("register_list_store")
            .expect("Couldn't get register_list_store");

        // Setup Actions

        let actions = gtk::gio::SimpleActionGroup::new();

        window.insert_action_group("disassemblercolumnview", Some(&actions));

        let columnview: gtk::ColumnViewColumn = builder
            .object("disassembly_address_column")
            .expect("Couldn't get disassembly_address_column");
        let action = gtk::gio::PropertyAction::new("show-address", &columnview, "visible");
        actions.add_action(&action);

        let columnview: gtk::ColumnViewColumn = builder
            .object("disassembly_instruction_column")
            .expect("Couldn't get disassembly_instruction_column");
        let action = gtk::gio::PropertyAction::new("show-instruction", &columnview, "visible");
        actions.add_action(&action);

        let columnview: gtk::ColumnViewColumn = builder
            .object("disassembly_opcode_column")
            .expect("Couldn't get disassembly_opcode_column");
        let action = gtk::gio::PropertyAction::new("show-opcode", &columnview, "visible");
        actions.add_action(&action);

        let columnview: gtk::ColumnViewColumn = builder
            .object("disassembly_operand_column")
            .expect("Couldn't get disassembly_operand_column");
        let action = gtk::gio::PropertyAction::new("show-operand", &columnview, "visible");
        actions.add_action(&action);

        // End Setup Actions

        scope.add_callback("pressed_cb", clone!(@strong btx, @strong disassembly_column_view => move |values| {
            let gesture = values[0].get::<gtk::GestureClick>().unwrap();
            let n_press = values[1].get::<i32>().unwrap();
            let x = values[2].get::<f64>().unwrap();
            let y = values[3].get::<f64>().unwrap();
            let listitem = values[4].get::<gtk::ListItem>().unwrap().item().unwrap();

            if gesture.current_button() == gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32 && n_press == 2 {
                let btx = btx.clone();
                let addr =
                    u32::from_str_radix(&listitem.property::<String>("address"), 16).unwrap();

                glib::MainContext::default().spawn_local(async move {
                    let _ = btx.send(BgEvent::BreakpointToggle(addr)).await;
                });

                if listitem.property::<String>("class").is_empty() {
                    listitem.set_property("class", "breakpoint-instruction");
                } else if listitem.property::<String>("class") == "breakpoint-instruction" {
                    listitem.set_property("class", "");
                }
            } else if gesture.current_button() == gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32 {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                let (newx, newy) = gesture.widget().translate_coordinates(&disassembly_column_view, x, y).unwrap();

                let menu_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
                let menu = gtk::Popover::builder()
                    .has_arrow(false)
                    .child(&menu_box)
                    .build();
                menu.set_parent(&disassembly_column_view);
                menu.set_pointing_to(Some(&gtk::gdk::Rectangle::new(
                    newx as i32,
                    newy as i32,
                    1,
                    1,
                )));

                let item = gtk::Button::builder()
                    .label("Copy address")
                    .has_frame(false)
                    .halign(gtk::Align::Start)
                    .build();
                item.connect_clicked(clone!(@weak menu, @weak listitem => move |_| {
                    let clipboard = menu.clipboard();
                    let value = listitem.property::<String>("address");
                    clipboard.set_text(&value);
                    menu.popdown();
                }));
                menu_box.append(&item);

                let item = gtk::Button::builder()
                    .label("Copy hex")
                    .has_frame(false)
                    .halign(gtk::Align::Start)
                    .build();
                item.connect_clicked(clone!(@weak menu, @weak listitem => move |_| {
                    let clipboard = menu.clipboard();
                    let value = listitem.property::<String>("instruction");
                    clipboard.set_text(&value);
                    menu.popdown();
                }));
                menu_box.append(&item);

                menu.popup();
            }
            None
        }));

        window.show();

        let registers = Registers::default();

        Self {
            tx,
            btx,
            disassembly_column_view,
            disassembly_list_store,
            register_store,
            registers,
            callstack_list_store,
            memory_list_store,
        }
    }

    pub fn update_callstack(&mut self, callstack: Callstack) {
        self.callstack_list_store.remove_all();

        for (i, address) in callstack.addresses.iter().enumerate() {
            let value = if i != 0 {
                format!("{:08x} address", address)
            } else {
                format!("{:08x} lr", address)
            };
            let row = CallstackObject::new(value);
            self.callstack_list_store.append(&row);
        }
    }

    pub fn update_disassembly(&mut self, disassembly: Disassembly) {
        let mut pc_row: u32 = 0;

        for (i, instr) in disassembly.instructions.iter().enumerate() {
            let addr_str = format!("{:08x}", instr.addr);
            let instr_str = format!("{:08x}", instr.instr.0);
            let widget_name = if disassembly.pc == instr.addr {
                pc_row = i as u32;
                String::from("current-instruction")
            } else if disassembly.breakpoints.contains(&instr.addr) {
                String::from("breakpoint-instruction")
            } else {
                String::new()
            };

            match self.disassembly_list_store.item(i as u32) {
                Some(object) => {
                    let item = object.downcast::<DisassembledInstruction>().unwrap();
                    // check if we need to update row
                    if item.property::<String>("address") != addr_str
                        || item.property::<String>("instruction") != instr_str
                        || item.property::<String>("class") != widget_name
                    {
                        // edit existing item instead of replace so list.scroll-to-item action
                        // functions properly
                        item.set_property("address", addr_str);
                        item.set_property("instruction", instr_str);
                        item.set_property("opcode", instr.mnemonic.to_string());
                        item.set_property("operands", instr.operands.to_string());
                        item.set_property("class", widget_name);
                    }
                }
                None => {
                    // create new row
                    let row = DisassembledInstruction::new(
                        addr_str,
                        instr_str,
                        instr.mnemonic.to_string(),
                        instr.operands.to_string(),
                        widget_name,
                    );

                    self.disassembly_list_store.append(&row);
                }
            }
        }

        // Scroll to pc row
        let mut child = self.disassembly_column_view.last_child();
        while let Some(widget) = child.clone() {
            if widget.is::<gtk::ListView>() {
                widget
                    .activate_action("list.scroll-to-item", Some(&(pc_row).to_variant()))
                    .expect("The action does not exist.");
                break;
            } else {
                child = widget.prev_sibling();
            }
        }
    }

    pub fn update_memory(&mut self, memory: Memory) {
        for (i, data) in memory.data.iter().enumerate() {
            let address = format!("{:08x}", data.0);
            let hex = format!(
                "{:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}",
                data.1[0], data.1[1], data.1[2], data.1[3], data.1[4], data.1[5], data.1[6], data.1[7], data.1[8], data.1[9], data.1[10], data.1[11], data.1[12], data.1[13], data.1[14], data.1[15]
            );
            let mut ascii = String::with_capacity(16);
            for ch in data.1.iter() {
                if ch.is_ascii_graphic() {
                    ascii.push(*ch as char);
                } else {
                    ascii.push('.');
                }
            }

            match self.memory_list_store.item(i as u32) {
                Some(object) => {
                    object.set_property("address", address);
                    object.set_property("hex", hex);
                    object.set_property("ascii", ascii);
                    //FixMe: update data
                }
                None => {
                    let object = MemoryDumpObject::new(address, hex, ascii);
                    self.memory_list_store.append(&object);
                }
            }
        }
    }

    pub fn update_registers(&mut self, regs: Registers) {
        self.register_store.clear();

        let color_updated = gtk::gdk::RGBA::new(0.0, 255.0, 0.0, 0.4);
        let color_same = gtk::gdk::RGBA::new(0.0, 0.0, 0.0, 0.0);

        // General-purpose registers
        for (i, reg) in regs.gpr.iter().enumerate() {
            let background_color = if self.registers.gpr[i] != *reg {
                &color_updated
            } else {
                &color_same
            };

            self.register_store.set(
                &self.register_store.append(),
                &[
                    (0, &format!("r{}", i)),
                    (1, &format!("{:08x}", reg)),
                    (2, &format!("{}", reg)),
                    (3, &background_color),
                ],
            )
        }

        // Floating-point registers
        for (i, reg) in regs.fpr.iter().enumerate() {
            let background_color = if self.registers.fpr[i].as_u64() != reg.as_u64() {
                &color_updated
            } else {
                &color_same
            };

            self.register_store.set(
                &self.register_store.append(),
                &[
                    (0, &format!("f{}", i)),
                    (1, &format!("{:08x}", reg.as_u64())),
                    (2, &format!("{}", reg.as_u64())),
                    (3, &background_color),
                ],
            )
        }

        for (name, value) in [
            ("XER", regs.spr[cpu::SPR_XER]),
            ("LR", regs.spr[cpu::SPR_LR]),
            ("CTR", regs.spr[cpu::SPR_CTR]),
            ("DSISR", regs.spr[cpu::SPR_DSISR]),
            ("DAR", regs.spr[cpu::SPR_DAR]),
            ("DEC", regs.spr[cpu::SPR_DEC]),
            ("SDR1", regs.spr[cpu::SPR_SDR1]),
            ("SRR0", regs.spr[cpu::SPR_SRR0]),
            ("SRR1", regs.spr[cpu::SPR_SRR1]),
            ("SPRG0", regs.spr[cpu::SPR_SPRG0]),
            ("SPRG1", regs.spr[cpu::SPR_SPRG0 + 1]),
            ("SPRG2", regs.spr[cpu::SPR_SPRG0 + 2]),
            ("SPRG3", regs.spr[cpu::SPR_SPRG0 + 3]),
            ("EAR", regs.spr[cpu::SPR_EAR]),
            ("TBL", regs.spr[cpu::SPR_TBL]),
            ("TBU", regs.spr[cpu::SPR_TBU]),
            ("PVR", regs.spr[cpu::SPR_PVR]),
            ("IBAT0U", regs.spr[cpu::SPR_IBAT0U]),
            ("IBAT0L", regs.spr[cpu::SPR_IBAT0L]),
            ("IBAT1U", regs.spr[cpu::SPR_IBAT1U]),
            ("IBAT1L", regs.spr[cpu::SPR_IBAT1L]),
            ("IBAT2U", regs.spr[cpu::SPR_IBAT2U]),
            ("IBAT2L", regs.spr[cpu::SPR_IBAT2L]),
            ("IBAT3U", regs.spr[cpu::SPR_IBAT3U]),
            ("IBAT3L", regs.spr[cpu::SPR_IBAT3L]),
            ("DBAT0U", regs.spr[cpu::SPR_DBAT0U]),
            ("DBAT0L", regs.spr[cpu::SPR_DBAT0L]),
            ("DBAT1U", regs.spr[cpu::SPR_DBAT1U]),
            ("DBAT1L", regs.spr[cpu::SPR_DBAT1L]),
            ("DBAT2U", regs.spr[cpu::SPR_DBAT2U]),
            ("DBAT2L", regs.spr[cpu::SPR_DBAT2L]),
            ("DBAT3U", regs.spr[cpu::SPR_DBAT3U]),
            ("DBAT3L", regs.spr[cpu::SPR_DBAT3L]),
            ("GQR0", regs.spr[cpu::SPR_GQR0]),
            ("GQR1", regs.spr[cpu::SPR_GQR0 + 1]),
            ("GQR2", regs.spr[cpu::SPR_GQR0 + 2]),
            ("GQR3", regs.spr[cpu::SPR_GQR0 + 3]),
            ("GQR4", regs.spr[cpu::SPR_GQR0 + 4]),
            ("GQR5", regs.spr[cpu::SPR_GQR0 + 5]),
            ("GQR6", regs.spr[cpu::SPR_GQR0 + 6]),
            ("GQR7", regs.spr[cpu::SPR_GQR0 + 7]),
            ("HID0", regs.spr[cpu::SPR_HID0]),
            ("HID1", regs.spr[cpu::SPR_HID1]),
            ("HID2", regs.spr[cpu::SPR_HID2]),
            ("WPAR", regs.spr[cpu::SPR_WPAR]),
            ("DMAU", regs.spr[cpu::SPR_DMAU]),
            ("DMAL", regs.spr[cpu::SPR_DMAU + 1]),
            ("UMMCR0", regs.spr[cpu::SPR_UMMCR0]),
            ("UPMC1", regs.spr[cpu::SPR_UPMC1]),
            ("UPMC2", regs.spr[cpu::SPR_UPMC2]),
            ("UPMC3", regs.spr[cpu::SPR_UPMC3]),
            ("UPMC4", regs.spr[cpu::SPR_UPMC4]),
            ("USIA", regs.spr[cpu::SPR_USIA]),
            ("UMMCR1", regs.spr[cpu::SPR_UMMCR1]),
            ("MMCR0", regs.spr[cpu::SPR_MMCR0]),
            ("PMC1", regs.spr[cpu::SPR_PMC1]),
            ("PMC2", regs.spr[cpu::SPR_PMC2]),
            ("PMC3", regs.spr[cpu::SPR_PMC3]),
            ("PMC4", regs.spr[cpu::SPR_PMC4]),
            ("SIA", regs.spr[cpu::SPR_SIA]),
            ("MMCR1", regs.spr[cpu::SPR_MMCR1]),
            ("IABR", regs.spr[cpu::SPR_IABR]),
            ("DABR", regs.spr[cpu::SPR_DABR]),
            ("L2CR", regs.spr[cpu::SPR_L2CR]),
            ("ICTC", regs.spr[cpu::SPR_ICTC]),
            ("THRM1", regs.spr[cpu::SPR_THRM1]),
        ]
        .iter()
        {
            self.register_store.set(
                &self.register_store.append(),
                &[
                    (0, name),
                    (1, &format!("{:08x}", value)),
                    (2, &format!("{}", value)),
                    (3, &color_same),
                ],
            )
        }
        self.registers = regs;
    }
}
