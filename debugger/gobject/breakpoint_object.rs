use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct BreakpointObject(ObjectSubclass<imp::BreakpointObject>);
}

impl BreakpointObject {
    pub fn new(
        type_: String,
        num: u32,
        address: String,
        condition: String,
        start_address: u32,
        end_address: u32,
    ) -> Self {
        Object::new(&[
            ("type", &type_),
            ("num", &num),
            ("address", &address),
            ("condition", &condition),
            ("startAddress", &start_address),
            ("endAddress", &end_address),
        ])
    }
}

mod imp {

    use std::cell::{Cell, RefCell};

    use gtk::glib;
    use gtk::glib::{ParamSpec, Value};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(Default)]
    pub struct BreakpointObject {
        type_: RefCell<String>,
        num: Cell<u32>,
        address: RefCell<String>,
        condition: RefCell<String>,
        start_address: Cell<u32>,
        end_address: Cell<u32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for BreakpointObject {
        const NAME: &'static str = "BreakpointObject";
        type Type = super::BreakpointObject;
        type ParentType = glib::Object;
        type Interfaces = ();
    }

    // Trait shared by all GObjects
    impl ObjectImpl for BreakpointObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::new(
                        "type",
                        "Type",
                        "breakpoint or watchpoint",
                        None, // Default value
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecUInt::builder("num").build(),
                    glib::ParamSpecString::new(
                        "address",
                        "address",
                        "breakpoint address",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "condition",
                        "Condition",
                        "Read, Write, ReadOrWrite",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecUInt::builder("startAddress").build(),
                    glib::ParamSpecUInt::builder("endAddress").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "type" => {
                    let type_ = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.type_.replace(type_);
                }
                "num" => {
                    let num = value.get().unwrap();
                    self.num.replace(num);
                }
                "address" => {
                    let address = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.address.replace(address);
                }
                "condition" => {
                    let condition = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.condition.replace(condition);
                }
                "startAddress" => {
                    let start_address = value.get().unwrap();
                    self.start_address.replace(start_address);
                }
                "endAddress" => {
                    let end_address = value.get().unwrap();
                    self.end_address.replace(end_address);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> Value {
            match pspec.name() {
                "type" => self.type_.borrow().to_value(),
                "num" => self.num.get().to_value(),
                "address" => self.address.borrow().to_value(),
                "condition" => self.condition.borrow().to_value(),
                "startAddress" => self.start_address.get().to_value(),
                "endAddress" => self.end_address.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
