use crate::gobject::{BreakpointObject, CallstackObject, DisassembledInstruction};
use crate::{
    BgEvent, BreakpointAccessType, BreakpointType, Breakpoints, Callstack, Disassembly, Event,
    Memory, Registers,
};
use async_channel::Sender;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;

pub struct App {
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
    disassembly_column_view: gtk::ColumnView,
    disassembly_list_store: gtk::gio::ListStore,
    register_store: gtk::ListStore,
    registers: Registers,
    callstack_list_store: gtk::gio::ListStore,
    breakpoint_list_store: gtk::gio::ListStore,
    button_continue: gtk::Button,
    label_memory_address: gtk::Label,
    label_memory_hex: gtk::Label,
    label_memory_ascii: gtk::Label,
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

        let button_new_breakpoint: gtk::Button = builder
            .object("button_new_breakpoint")
            .expect("Couldn't get button_new_breakpoint");

        let button_clear_breakpoint: gtk::Button = builder
            .object("button_clear_breakpoint")
            .expect("Couldn't get button_clear_breakpoint");

        let button_delete_breakpoint: gtk::Button = builder
            .object("button_delete_breakpoint")
            .expect("Couldn't get button_delete_breakpoint");

        let dialog_breakpoint: gtk::Dialog = builder
            .object("new_breakpoint_dialog")
            .expect("Couldn't get new_breakpoint_dialog");

        button_step.connect_clicked(clone!(@strong btx => move |_| {
            let btx = btx.clone();
            glib::MainContext::default().spawn_local(async move {
                let _ = btx.send(BgEvent::Step).await;
            });
        }));

        button_continue.connect_clicked(clone!(@strong btx, @weak button_continue => move |_| {
            let btx = btx.clone();
            button_continue.set_sensitive(false);
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

        button_new_breakpoint.connect_clicked(clone!(@strong dialog_breakpoint => move |_| {
            dialog_breakpoint.show();
        }));

        let breakpoint_list_store = gtk::gio::ListStore::new(BreakpointObject::static_type());

        let breakpoint_selection = gtk::SingleSelection::new(Some(&breakpoint_list_store));

        button_delete_breakpoint.connect_clicked(
            clone!(@strong btx, @weak breakpoint_selection => move |_| {
                if let Some(bp) = breakpoint_selection.selected_item() {
                    let btx = btx.clone();
                    let bg_event = match bp.property::<String>("type").as_str() {
                        "Instruction" => {
                            BgEvent::BreakpointRemove(BreakpointType::Break, bp.property::<u32>("num"))
                        },
                        "Memory" => {
                            BgEvent::BreakpointRemove(BreakpointType::Watch, bp.property::<u32>("num"))
                        }
                        _ => unreachable!(),
                    };

                    glib::MainContext::default().spawn_local(async move {
                        let _ = btx.send(bg_event).await;
                    });
                }
            }),
        );

        button_clear_breakpoint.connect_clicked(clone!(@strong btx => move |_| {
            let btx = btx.clone();
            glib::MainContext::default().spawn_local(async move {
                let _ = btx.send(BgEvent::BreakpointClear).await;
            });
        }));

        let check_button_instruction: gtk::CheckButton = builder
            .object("instruction_check_button")
            .expect("Couldn't get instruction_check_button");

        let entry_instruction_breakpoint: gtk::Entry = builder
            .object("entry_instruction_breakpoint")
            .expect("Couldn't get entry_instruction_breakpoint");

        let entry_memory_breakpoint_start: gtk::Entry = builder
            .object("entry_memory_breakpoint_start")
            .expect("Couldn't get entry_memory_breakpoint_start");

        let entry_memory_breakpoint_end: gtk::Entry = builder
            .object("entry_memory_breakpoint_end")
            .expect("Couldn't get entry_memory_breakpoint_end");

        let memory_condition_read_check_button: gtk::CheckButton = builder
            .object("memory_condition_read_check_button")
            .expect("Couldn't get memory_condition_read_check_button");

        let memory_condition_write_check_button: gtk::CheckButton = builder
            .object("memory_condition_write_check_button")
            .expect("Couldn't get memory_condition_write_check_button");

        let memory_condition_read_write_check_button: gtk::CheckButton = builder
            .object("memory_condition_read_write_check_button")
            .expect("Couldn't get memory_condition_read_write_check_button");

        check_button_instruction.connect_toggled(
            clone!(@weak entry_instruction_breakpoint, @weak entry_memory_breakpoint_start, @weak entry_memory_breakpoint_end, @weak memory_condition_read_check_button, @weak memory_condition_write_check_button, @weak memory_condition_read_write_check_button  => move |check_button_instruction| {
                if check_button_instruction.is_active() {
                    entry_instruction_breakpoint.set_sensitive(true);
                    entry_memory_breakpoint_start.set_sensitive(false);
                    entry_memory_breakpoint_end.set_sensitive(false);
                    memory_condition_read_check_button.set_sensitive(false);
                    memory_condition_write_check_button.set_sensitive(false);
                    memory_condition_read_write_check_button.set_sensitive(false);
                } else {
                    entry_instruction_breakpoint.set_sensitive(false);
                    entry_memory_breakpoint_start.set_sensitive(true);
                    entry_memory_breakpoint_end.set_sensitive(true);
                    memory_condition_read_check_button.set_sensitive(true);
                    memory_condition_write_check_button.set_sensitive(true);
                    memory_condition_read_write_check_button.set_sensitive(true);
                }
            }),
        );

        dialog_breakpoint.connect_response(
            clone!(@strong btx, @weak entry_instruction_breakpoint, @weak entry_memory_breakpoint_start, @weak entry_memory_breakpoint_end, @weak memory_condition_read_check_button, @weak memory_condition_write_check_button, @weak memory_condition_read_write_check_button => move |dialog_breakpoint, response| {
                if response == gtk::ResponseType::Ok {
                    let btx = btx.clone();

                    if check_button_instruction.is_active() {
                        // breakpoint
                        let addr:u32 = u32::from_str_radix(&entry_instruction_breakpoint.buffer().text(), 16).unwrap();

                        glib::MainContext::default().spawn_local(async move {
                            let _ = btx.send(BgEvent::Breakpoint(BreakpointType::Break, addr, 0, false, false)).await;
                        });
                    } else {
                        // Watchpoint
                        let start_addr:u32 = u32::from_str_radix(&entry_memory_breakpoint_start.buffer().text(), 16).unwrap();
                        let end_addr:u32 = u32::from_str_radix(&entry_memory_breakpoint_end.buffer().text(), 16).unwrap();

                        let (break_on_write, break_on_read) = if memory_condition_read_check_button.is_active() {
                            (false, true)
                        } else if memory_condition_write_check_button.is_active() {
                            (true, false)
                        } else if memory_condition_read_write_check_button.is_active() {
                            (true, true)
                        } else {
                            unreachable!()
                        };

                        glib::MainContext::default().spawn_local(async move {
                            let _ = btx.send(BgEvent::Breakpoint(BreakpointType::Watch, start_addr, end_addr, break_on_write, break_on_read)).await;
                        });
                    }
                }

                dialog_breakpoint.hide();

                // reset fields
                check_button_instruction.set_active(true);
                entry_instruction_breakpoint.buffer().set_text("");
                entry_memory_breakpoint_start.buffer().set_text("");
                entry_memory_breakpoint_end.buffer().set_text("");
                memory_condition_read_check_button.set_active(true);
            }));

        let disassembly_list_store =
            gtk::gio::ListStore::new(DisassembledInstruction::static_type());

        let sel = gtk::NoSelection::new(Some(&disassembly_list_store));
        let disassembly_column_view: gtk::ColumnView = builder
            .object("disassembly_column_view")
            .expect("Couldn't get disassembly_column_view");

        disassembly_column_view.set_model(Some(&sel));

        let callstack_list_store = gtk::gio::ListStore::new(CallstackObject::static_type());

        let sel = gtk::NoSelection::new(Some(&callstack_list_store));
        let callstack_list_view: gtk::ListView = builder
            .object("callstack_list_view")
            .expect("Couldn't get callstack_list_view");
        callstack_list_view.set_model(Some(&sel));

        let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");

        window.set_application(Some(app));

        let register_store: gtk::ListStore = builder
            .object("register_list_store")
            .expect("Couldn't get register_list_store");

        //let breakpoint_list_store = gtk::gio::ListStore::new(BreakpointObject::static_type());

        //let breakpoint_selection = gtk::SingleSelection::new(Some(&breakpoint_list_store));
        let breakpoint_column_view: gtk::ColumnView = builder
            .object("breakpoint_column_view")
            .expect("Couldn't get breakpoint_column_view");

        breakpoint_column_view.set_model(Some(&breakpoint_selection));

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

        let memory_jump_entry: gtk::Entry = builder
            .object("memory_jump_entry")
            .expect("Couldn't get memory_jump_entry");

        memory_jump_entry.connect_activate(clone!(@strong btx => move |entry| {
            let btx = btx.clone();
            let text = entry.buffer().text();
            if let Ok(addr) = u32::from_str_radix(text.trim_start_matches("0x"), 16) {
                    glib::MainContext::default().spawn_local(async move {
                        let _ = btx.send(BgEvent::MemoryDump(addr)).await;
                    });
            }
        }));

        let label_memory_address: gtk::Label = builder
            .object("label_memory_address")
            .expect("Couldn't get label_memory_address");

        let label_memory_hex: gtk::Label = builder
            .object("label_memory_hex")
            .expect("Couldn't get label_memory_hex");

        let label_memory_ascii: gtk::Label = builder
            .object("label_memory_ascii")
            .expect("Couldn't get label_memory_ascii");

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
            breakpoint_list_store,
            button_continue,
            label_memory_address,
            label_memory_hex,
            label_memory_ascii,
        }
    }

    pub fn paused(&mut self) {
        self.button_continue.set_sensitive(true);
    }

    pub fn update_breakpoints(&mut self, bps: Breakpoints) {
        self.breakpoint_list_store.remove_all();

        for bp in bps.breakpoints.iter() {
            // create new row
            let (type_, addr) = match bp.type_ {
                BreakpointType::Break => (
                    String::from("Instruction"),
                    format!("{:08x}", bp.start_address),
                ),
                BreakpointType::Watch => {
                    let addr = if bp.start_address < bp.end_address {
                        format!("{:08x}-{:08x}", bp.start_address, bp.end_address)
                    } else {
                        format!("{:08x}", bp.start_address)
                    };
                    (String::from("Memory"), addr)
                }
            };

            let condition = match bp.access_type {
                BreakpointAccessType::Write => String::from("W"),
                BreakpointAccessType::Read => String::from("R"),
                BreakpointAccessType::ReadWrite => String::from("RW"),
            };
            let row = BreakpointObject::new(
                type_,
                bp.num,
                addr,
                condition,
                bp.start_address,
                bp.end_address,
            );

            self.breakpoint_list_store.append(&row);
        }
    }

    pub fn update_callstack(&mut self, callstack: Callstack) {
        self.callstack_list_store.remove_all();

        if callstack.addresses.is_empty() {
            self.callstack_list_store
                .append(&CallstackObject::new("Invalid Callstack".to_string()));
        } else {
            for (i, address) in callstack.addresses.iter().enumerate() {
                let value = if i != 0 {
                    format!("{address:08x} address")
                } else {
                    format!("{address:08x} lr")
                };
                let row = CallstackObject::new(value);
                self.callstack_list_store.append(&row);
            }
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
        let (mut address, mut hex, mut ascii) = (Vec::new(), Vec::new(), Vec::new());

        for data in memory.data.iter() {
            address.push(format!("{:08x}", data.0));
            hex.push(format!(
                "{:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}",
                data.1[0], data.1[1], data.1[2], data.1[3], data.1[4], data.1[5], data.1[6], data.1[7], data.1[8], data.1[9], data.1[10], data.1[11], data.1[12], data.1[13], data.1[14], data.1[15]
            ));

            let mut a = String::with_capacity(16);
            for ch in data.1.iter() {
                if ch.is_ascii_graphic() {
                    a.push(*ch as char);
                } else {
                    a.push('.');
                }
            }
            ascii.push(a);
        }

        self.label_memory_address.set_text(&address.join("\n"));
        self.label_memory_hex.set_text(&hex.join("\n"));
        self.label_memory_ascii.set_text(&ascii.join("\n"));
    }

    pub fn update_registers(&mut self, regs: Registers) {
        self.register_store.clear();

        let color_updated = gtk::gdk::RGBA::new(0.0, 255.0, 0.0, 0.4);
        let color_same = gtk::gdk::RGBA::new(0.0, 0.0, 0.0, 0.0);
        let weight = 700;

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
                    (0, &format!("r{i}")),
                    (1, &format!("{reg:08x}")),
                    (2, &format!("{reg}")),
                    (3, &background_color),
                    (4, &weight),
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
                    (0, &format!("f{i}")),
                    (1, &format!("{:016x}", reg.as_u64())),
                    (2, &format!("{}", reg.as_u64())),
                    (3, &background_color),
                    (4, &weight),
                ],
            )
        }

        // Special-purpose registers u32
        for (i, spr_32) in regs.spr_32.iter().enumerate() {
            let background_color = if self.registers.spr_32[i].1 != spr_32.1 {
                &color_updated
            } else {
                &color_same
            };

            self.register_store.set(
                &self.register_store.append(),
                &[
                    (0, &spr_32.0),
                    (1, &format!("{:08x}", spr_32.1)),
                    (2, &format!("{}", spr_32.1)),
                    (3, &background_color),
                    (4, &weight),
                ],
            )
        }

        // Special Purpose Register u64
        for (i, spr_64) in regs.spr_64.iter().enumerate() {
            let background_color = if self.registers.spr_64[i].1 != spr_64.1 {
                &color_updated
            } else {
                &color_same
            };

            self.register_store.set(
                &self.register_store.append(),
                &[
                    (0, &spr_64.0),
                    (1, &format!("{:016x}", spr_64.1)),
                    (2, &format!("{}", spr_64.1)),
                    (3, &background_color),
                    (4, &weight),
                ],
            )
        }

        self.registers = regs;
    }
}
