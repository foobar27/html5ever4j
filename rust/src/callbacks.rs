use jni_sys::{jmethodID, jobject, jboolean, JNIEnv};
use jni::{JObject, JClass, string_to_jstring, jobject_vec_to_jobjectarray};
use algorithms::{Attribute,Callback};
use libc::c_uint;

static PACKAGE: &'static str = "com.github.foobar27.html5ever4j"; // TODO duplicate code

// TODO create macro for the following structure:
// (potentially with string/jboolean/... serialization?)

pub struct JavaCallbackClass {
    string_class: JClass,
    set_doc_type_method: jmethodID,
    create_text_method: jmethodID,
    create_comment_method: jmethodID,
    create_normal_element_method: jmethodID,
    create_script_element_method: jmethodID,
    create_template_element_method: jmethodID,
    create_annotation_xml_element_method: jmethodID,
}

impl JavaCallbackClass {

    unsafe fn new(jre: *mut JNIEnv, class: JClass) -> Result<JavaCallbackClass, ()> {
        return Ok(JavaCallbackClass {
            string_class: try!(try!(JClass::load(jre, "java.lang", "String")).create_global_ref(jre)),
            set_doc_type_method: try!(class.get_method_id(jre, "setDocType", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V")),
            create_text_method: try!(class.get_method_id(jre, "createText", "(Ljava/lang/String;)V")),
            create_comment_method: try!(class.get_method_id(jre, "createComment", "(Ljava/lang/String;)V")),
            create_normal_element_method: try!(class.get_method_id(jre, "createNormalElement", "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V")),
            create_script_element_method: try!(class.get_method_id(jre, "createScriptElement", "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Z)V")),
            create_template_element_method: try!(class.get_method_id(jre, "createTemplateElement", "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V")),
            create_annotation_xml_element_method: try!(class.get_method_id(jre, "createAnnotationXmlElement", "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Z)V")),
        })
    }

    pub unsafe fn load(jre: *mut JNIEnv) -> Result<JavaCallbackClass, ()> {
        return JavaCallbackClass::new(jre, try!(JClass::load(jre, PACKAGE, "Parser$CallBack")));
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

unsafe fn flatten_attributes(jre: *mut JNIEnv, attributes: &Vec<Attribute>) -> Vec<jobject> {
    let mut flat_attributes = Vec::<jobject>::new();
    for a in attributes {
        flat_attributes.push(string_to_jstring(jre, a.ns.clone())); // TODO remove clone!
        flat_attributes.push(string_to_jstring(jre, a.key.clone())); // TODO remove clone!
        flat_attributes.push(string_to_jstring(jre, a.value.clone())); // TODO remove clone!
    }
    return flat_attributes;
}

impl Callback for JavaCallbackObject {

    fn set_doc_type(&self, name: String, public: String, system: String) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.set_doc_type_method,
                 string_to_jstring(self.jre, name),
                 string_to_jstring(self.jre, public),
                 string_to_jstring(self.jre, system));
        }
    }

    fn create_text(&self, text: String) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_text_method,
                 string_to_jstring(self.jre, text));
        }
    }

    fn create_comment(&self, comment: String) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_comment_method,
                 string_to_jstring(self.jre, comment));
                                                      }
    }
   
    fn create_normal_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_normal_element_method,
                 string_to_jstring(self.jre, ns), // TODO profit from QualName -> same jstring
                 string_to_jstring(self.jre, tag_name), // TODO profit from QualName -> same jstring 
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()));
        }
    }

    fn create_script_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>, already_started: bool) {
        let already_started: jboolean = if already_started {1} else {0};
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_script_element_method,
                 string_to_jstring(self.jre, ns), // TODO profit from QualName -> same jstring
                 string_to_jstring(self.jre, tag_name), // TODO profit from QualName -> same jstring 
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()),
                 already_started as c_uint);
        }
    }

    fn create_template_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_template_element_method,
                 string_to_jstring(self.jre, ns), // TODO profit from QualName -> same jstring
                 string_to_jstring(self.jre, tag_name), // TODO profit from QualName -> same jstring 
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()));
        }
    }

    fn create_annotation_xml_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>, b: bool) {
        let b: jboolean = if b {1} else {0};
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_annotation_xml_element_method,
                 string_to_jstring(self.jre, ns), // TODO profit from QualName -> same jstring
                 string_to_jstring(self.jre, tag_name), // TODO profit from QualName -> same jstring 
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()),
                 b as c_uint);
        }
    }
}
