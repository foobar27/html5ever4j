use jni_sys::{jmethodID, jobject, JNIEnv};
use jni::{JObject, JClass, string_to_jstring};
use algorithms::{Attribute,Callback};

struct JavaCallbackClass {
    string_class: JClass,
    set_doc_type_method: jmethodID,
    create_text_method: jmethodID,
    create_comment_method: jmethodID,
    create_normal_element_method: jmethodID,
}

impl JavaCallbackClass {

    unsafe fn new(jre: *mut JNIEnv, class: JClass) -> Result<JavaCallbackClass, ()> {
        return Ok(JavaCallbackClass {
            string_class: try!(try!(JClass::load(jre, "java.lang", "String")).create_global_ref(jre)),
            set_doc_type_method: try!(class.get_method_id(jre, "setDocType".to_string(), "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V".to_string())),
            create_text_method: try!(class.get_method_id(jre, "createText".to_string(), "(Ljava/lang/String;)V".to_string())),
            create_comment_method: try!(class.get_method_id(jre, "createComment".to_string(), "(Ljava/lang/String;)V".to_string())),
            create_normal_element_method: try!(class.get_method_id(jre, "createNormalElement".to_string(), "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V".to_string()))
        })
    }

    
}

struct JavaCallbackObject {
    jre: *mut JNIEnv,
    class: JavaCallbackClass,
    object: JObject,
}

impl JavaCallbackObject {

    fn new(jre: *mut JNIEnv, class: JavaCallbackClass, object: JObject) -> JavaCallbackObject {
        return JavaCallbackObject {
            jre: jre,
            class: class,
            object: object,
        }
    }
    
}

impl Callback for JavaCallbackObject {
    
    fn set_doc_type(&self, name: String, public: String, system: String) {
        unsafe {
            ((**self.jre).CallVoidMethod)(
                self.jre,
                self.object.object,
                self.class.set_doc_type_method,
                string_to_jstring(self.jre, name),
                string_to_jstring(self.jre, public),
                string_to_jstring(self.jre, system));
        }
    }

    fn create_text(&self, text: String) {
        unsafe {
            ((**self.jre).CallVoidMethod)(
                self.jre,
                self.object.object,
                self.class.create_text_method,
                string_to_jstring(self.jre, text));
        }
    }

    fn create_comment(&self, comment: String) {
        unsafe {
            ((**self.jre).CallVoidMethod)(
                self.jre,
                self.object.object,
                self.class.create_comment_method,
                string_to_jstring(self.jre, comment));
                                                      }
    }

    fn create_normal_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>) {
        unsafe {
            let mut flat_attributes = Vec::<jobject>::new();
            for a in attributes {
                flat_attributes.push(string_to_jstring(self.jre, a.ns));
                flat_attributes.push(string_to_jstring(self.jre, a.key));
                flat_attributes.push(string_to_jstring(self.jre, a.value));
            }
            ((**self.jre).CallVoidMethod)(
                self.jre,
                self.object.object,
                self.class.create_normal_element_method,
                string_to_jstring(self.jre, ns),
                string_to_jstring(self.jre, tag_name),
                flat_attributes);
        }
    }
    
}
