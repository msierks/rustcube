use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct MemoryDumpObject(ObjectSubclass<imp::MemoryDumpObject>);
}

impl MemoryDumpObject {
    pub fn new(address: String, hex: String, ascii: String) -> Self {
        Object::new(&[("address", &address), ("hex", &hex), ("ascii", &ascii)])
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
    pub struct MemoryDumpObject {
        address: RefCell<String>,
        hexdump: RefCell<String>,
        ascii: RefCell<String>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for MemoryDumpObject {
        const NAME: &'static str = "MemoryDumpObject";
        type Type = super::MemoryDumpObject;
        type ParentType = glib::Object;
        type Interfaces = ();
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MemoryDumpObject {
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
                        "hex",
                        "Hex",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "ascii",
                        "ascii",
                        "Whether to auto-update or not",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "address" => {
                    let address = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.address.replace(address);
                }
                "hex" => {
                    let hexdump = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.hexdump.replace(hexdump);
                }
                "ascii" => {
                    let ascii = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.ascii.replace(ascii);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> Value {
            match pspec.name() {
                "address" => self.address.borrow().to_value(),
                "hex" => self.hexdump.borrow().to_value(),
                "ascii" => self.ascii.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
