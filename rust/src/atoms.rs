use std::option;
use std::ptr;
use jni::string_to_jstring;
use jni_sys::{jstring,JNIEnv};

use html5ever_atoms::Namespace;

pub struct JAtom {
    pub id: i32,
    pub string: jstring
}

impl JAtom {

    fn new_known(id: i32) -> JAtom {
        return JAtom {
            id: id,
            string: ptr::null_mut()
        }
    }

    fn new_unknown(jre: *mut JNIEnv, s: String) -> JAtom {
        unsafe {
            return JAtom {
                id: -1,
                string: string_to_jstring(jre, s)
            }
        }
    }
    
}

pub fn translate_namespace(jre: *mut JNIEnv, atom: &Namespace) -> JAtom {
    match *atom {
        ns!() => JAtom::new_known(0),
        ns!(*) => JAtom::new_known(1),
        ns!(html) => JAtom::new_known(2),
        ns!(xml) => JAtom::new_known(3),
        ns!(xmlns) => JAtom::new_known(4),
        ns!(xlink) => JAtom::new_known(5),
        ns!(svg) => JAtom::new_known(6),
        ns!(mathml) => JAtom::new_known(7),
        _ => JAtom::new_unknown(jre, atom.to_string())
    }
}
