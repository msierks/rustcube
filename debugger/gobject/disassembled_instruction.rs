use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct DisassembledInstruction(ObjectSubclass<imp::DisassembledInstruction>);
}

impl DisassembledInstruction {
    pub fn new(
        address: String,
        instruction: String,
        opcode: String,
        operands: String,
        class: String,
    ) -> Self {
        Object::new(&[
            ("address", &address),
            ("instruction", &instruction),
            ("opcode", &opcode),
            ("operands", &operands),
            ("class", &class),
        ])
        .expect("Failed to create `DisassembledInstruction`.")
    }
}

mod imp {

    use std::cell::RefCell;

    use gtk::glib;
    use gtk::glib::{ParamSpec, Value};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(Default)]
    pub struct DisassembledInstruction {
        address: RefCell<String>,
        instruction: RefCell<String>,
        opcode: RefCell<String>,
        operands: RefCell<String>,
        class: RefCell<String>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for DisassembledInstruction {
        const NAME: &'static str = "DisassembledInstruction";
        type Type = super::DisassembledInstruction;
        type ParentType = glib::Object;
        type Interfaces = ();
    }

    // Trait shared by all GObjects
    impl ObjectImpl for DisassembledInstruction {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::new(
                        "address",
                        "Address",
                        "Whether to auto-update or not",
                        None, // Default value
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "instruction",
                        "Instruction",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "opcode",
                        "Opcode",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "operands",
                        "Operands",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "class",
                        "Class",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "address" => {
                    let address = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.address.replace(address);
                }
                "instruction" => {
                    let instruction = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.instruction.replace(instruction);
                }
                "opcode" => {
                    let opcode = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.opcode.replace(opcode);
                }
                "operands" => {
                    let operands = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.operands.replace(operands);
                }
                "class" => {
                    let class = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.class.replace(class);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> Value {
            match pspec.name() {
                "address" => self.address.borrow().to_value(),
                "instruction" => self.instruction.borrow().to_value(),
                "opcode" => self.opcode.borrow().to_value(),
                "operands" => self.operands.borrow().to_value(),
                "class" => self.class.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}
