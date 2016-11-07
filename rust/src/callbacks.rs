use jni_sys::{jmethodID, jobject, jboolean, JNIEnv};
use jni::{JObject, JClass, string_to_jstring, jobject_vec_to_jobjectarray};
use algorithms::{Attribute,Callback};
use libc::c_uint;
use std::rc::Rc;

use html5ever_atoms::{LocalName, Namespace};
use atoms::{translate_localname, translate_namespace};
    
static PACKAGE: &'static str = "com.github.foobar27.html5ever4j"; // TODO duplicate code

// TODO create macro for the following structure:
// (potentially with string/jboolean/... serialization?)

pub struct JavaCallbackClass {
    string_class: JClass,
    pre_order_visit_method: jmethodID,
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
            pre_order_visit_method: try!(class.get_method_id(jre, "preOrderVisit", "()V")),
            set_doc_type_method: try!(class.get_method_id(jre, "setDocType", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V")),
            create_text_method: try!(class.get_method_id(jre, "createText", "(Ljava/lang/String;)V")),
            create_comment_method: try!(class.get_method_id(jre, "createComment", "(Ljava/lang/String;)V")),
            create_normal_element_method: try!(class.get_method_id(jre, "createNormalElement", "(ILjava/lang/String;ILjava/lang/String;[Ljava/lang/String;)V")),
            create_script_element_method: try!(class.get_method_id(jre, "createScriptElement", "(ILjava/lang/String;ILjava/lang/String;[Ljava/lang/String;Z)V")),
            create_template_element_method: try!(class.get_method_id(jre, "createTemplateElement", "(ILjava/lang/String;ILjava/lang/String;[Ljava/lang/String;)V")),
            create_annotation_xml_element_method: try!(class.get_method_id(jre, "createAnnotationXmlElement", "(ILjava/lang/String;ILjava/lang/String;[Ljava/lang/String;Z)V")),
        })
    }

    pub unsafe fn load(jre: *mut JNIEnv) -> Result<JavaCallbackClass, ()> {
        return JavaCallbackClass::new(jre, try!(JClass::load(jre, PACKAGE, "Parser$CallBack")));
    }
    
    
}

pub struct JavaCallbackObject {
    jre: *mut JNIEnv,
    class: Rc<JavaCallbackClass>,
    object: JObject,
}

impl JavaCallbackObject {

    pub fn new(jre: *mut JNIEnv, class: Rc<JavaCallbackClass>, object: JObject) -> JavaCallbackObject {
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

    fn pre_order_visit(&self) {
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.pre_order_visit_method);
        }
    }
    
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
   
    fn create_normal_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>) {
        let ns = translate_namespace(self.jre, ns);
        let tag = translate_localname(self.jre, tag);
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_normal_element_method,
                 ns.id,
                 ns.string,
                 tag.id,
                 tag.string,
                 jobject_vec_to_jobjectarray(
                     self.jre,
                     &flatten_attributes(self.jre, &attributes),
                     self.class.string_class.clone())
            .unwrap());
            // TODO dangerous unwrap
        }
    }

    fn create_script_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>, already_started: bool) {
        let already_started: jboolean = if already_started {1} else {0};
        let ns = translate_namespace(self.jre, ns);
        let tag = translate_localname(self.jre, tag);
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_script_element_method,
                 ns.id,
                 ns.string,
                 tag.id,
                 tag.string,
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()).unwrap(), // TODO dangerous unwrap
                 already_started as c_uint);
        }
    }

    fn create_template_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>) {
        let ns = translate_namespace(self.jre, ns);
        let tag = translate_localname(self.jre, tag);
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_template_element_method,
                 ns.id,
                 ns.string,
                 tag.id,
                 tag.string,
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()).unwrap());
            // TODO dangerous unwrap
        }
    }

    fn create_annotation_xml_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>, b: bool) {
        let b: jboolean = if b {1} else {0};
        let ns = translate_namespace(self.jre, ns);
        let tag = translate_localname(self.jre, tag);
        unsafe {
            jni!(self.jre, CallVoidMethod,
                 self.object.object,
                 self.class.create_annotation_xml_element_method,
                 ns.id,
                 ns.string,
                 tag.id,
                 tag.string,
                 jobject_vec_to_jobjectarray(self.jre, &flatten_attributes(self.jre, &attributes), self.class.string_class.clone()).unwrap(), // TODO dangerous unwrap
                 b as c_uint);
        }
    }
}
