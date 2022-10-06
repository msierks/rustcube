use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CallstackObject(ObjectSubclass<imp::CallstackObject>);
}

impl CallstackObject {
    pub fn new(value: String) -> Self {
        Object::new(&[("value", &value)]).expect("Failed to create `CallstackObject`.")
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
    pub struct CallstackObject {
        value: RefCell<String>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CallstackObject {
        const NAME: &'static str = "CallstackObject";
        type Type = super::CallstackObject;
        type ParentType = glib::Object;
        type Interfaces = ();
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CallstackObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpecString::new(
                    "value",
                    "Value",
                    "Whether to auto-update or not",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "value" => {
                    let value = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.value.replace(value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> Value {
            match pspec.name() {
                "value" => self.value.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}
