use std::ptr;
use std::rc::Rc;
use jni_sys::{jlong,jint,jstring,jclass,jobject,jmethodID,jfieldID,JNIEnv};
use std::collections::HashMap;
use std::marker::PhantomData;

use helper::{to_ptr,to_string};

fn mangled_class_name(package: &str, name: &str) -> String {
    let mut result: String = package.chars()
        .map(|c| if c=='.' {'/'} else {c})
        .collect();
    // TODO replace . by /
    result.push_str("/");
    result.push_str(name);
    return result;
}

pub fn box_to_jlong<T>(t: T) -> jlong {
    return Box::into_raw(Box::new(t)) as jlong;
}

pub unsafe fn free_struct<T>(ptr: jlong) {
    if ptr == 0 {
        return;
    }
    // This will take ownership of the pointer, and free it properly.
    Box::from_raw(ptr as *mut T);
}

// TODO create struct JString
pub unsafe fn jstring_to_string(jre: *mut JNIEnv, s: jstring) -> String {
    let chars = ((**jre).GetStringUTFChars)(jre, s, ptr::null_mut());
    let result = to_string(chars);
    ((**jre).ReleaseStringUTFChars)(jre, s, chars);
    return result
}

pub unsafe fn string_to_jstring(jre: *mut JNIEnv, s: String) -> jstring {
    return ((**jre).NewStringUTF)(jre, to_ptr(s));
}

// TODO reset exceptions
fn jfield_id_result(id: jfieldID) -> Result<jfieldID, ()> {
    if id.is_null() {
        return Err(())
    } else {
        return Ok(id)
    }
}

fn jmethod_id_result(id: jmethodID) -> Result<jmethodID, ()> {
    if id.is_null() {
        return Err(())
    } else {
        return Ok(id)
    }
}

/// A local reference to a java object.
#[derive(Debug, Clone)]
pub struct JObject {
    object: jobject,
    owned: bool
}

impl JObject {

    pub fn new_owned(object: jobject) -> JObject {
        return JObject {
            object: object,
            owned: true
        }
    }

    // TODO maybe reuse somehow the Borrow trait?
    pub fn new_borrowed(object: jobject) -> JObject {
        return JObject {
            object: object,
            owned: false
        }
    }

    // TODO make generic
    unsafe fn get_boolean_field(&self, jre: *mut JNIEnv, id: jfieldID) -> bool {
        return ((**jre).GetBooleanField)(jre, self.object, id) > 0
    }

    // TODO make generic
    unsafe fn get_object_field(&self, jre: *mut JNIEnv, id: jfieldID) -> JObject {
        return JObject::new_owned(((**jre).GetObjectField)(jre, self.object, id))
    }

    // TODO make generic: .call_method<JObject>(id)?
    // TODO allow parameters
    pub unsafe fn call_object_method(&self, jre: *mut JNIEnv, method_id: jmethodID) -> JObject {
        return JObject::new_owned(
            ((**jre).CallObjectMethod)(
                jre,
                self.object,
                method_id));
    }

    pub unsafe fn call_int_method(&self, jre: *mut JNIEnv, id: jmethodID) -> jint {
        return ((**jre).CallIntMethod)(jre, self.object, id);
    }


}

#[derive(Debug, Clone)]
pub struct JClass {
    class: jclass,
    owned: bool,
}

impl JClass {

    pub fn new_owned(class: jclass) -> JClass {
        return JClass {
            class: class,
            owned: true,
        }
    }

    pub fn type_signature(package: &str, name: &str) -> String {
        let mut result = String::from("L");
        result.push_str(&mangled_class_name(package, name));
        result.push_str(";");
        return result;
    }
    
    pub unsafe fn load(jre: *mut JNIEnv, package: &str, name: &str) -> Result<JClass, ()> {
        let class = ((**jre).FindClass)(jre, to_ptr(mangled_class_name(package, name)));
        if class.is_null() {
            return Err(())
        }
        return Ok(JClass {
            class: class,
            owned: true
        })
    }
    
    pub fn new_borrowed(class: jclass) -> JClass {
        return JClass {
            class: class,
            owned: true,
        }
    }

    unsafe fn get_field_id(&self, jre: *mut JNIEnv, name: &str, sig: &str) -> Result<jfieldID, ()> {
        let result = ((**jre).GetFieldID)(
            jre,
            self.class,
            to_ptr(name.to_string()),
            to_ptr(sig.to_string()));
        return jfield_id_result(result);
    }
   
    pub unsafe fn get_method_id(&self, jre: *mut JNIEnv, name: String, signature: String) -> Result<jmethodID, ()> {
        let result = ((**jre).GetMethodID)(
            jre,
            self.class,
            to_ptr(name),
            to_ptr(signature));
        return jmethod_id_result(result);
    }
    
    pub unsafe fn get_static_field_id(&self, jre: *mut JNIEnv, name: &str, signature: String) -> Result<jfieldID, ()>{
        let result = ((**jre).GetStaticFieldID)(
            jre,
            self.class,
            to_ptr(name.to_owned()),
            to_ptr(signature.to_owned()));
        return jfield_id_result(result)
    }

    // TODO make generic
    pub unsafe fn get_static_object_field(&self, jre: *mut JNIEnv, id: jfieldID) -> JObject {
        return JObject::new_owned(
            ((**jre).GetStaticObjectField)(
                jre,
                self.class,
                id))
    }

}

pub struct FieldMetaData {
    field_id: jfieldID
}

impl FieldMetaData {
    pub unsafe fn new(jre: *mut JNIEnv, name: &str, signature: &str, class: JClass) -> Result<FieldMetaData, ()> {
        return Ok(FieldMetaData {
            field_id: try!(class.get_field_id(jre, name, signature)),
        })
    }
}

pub trait FieldGetter<T> {
    unsafe fn get(&self, jre: *mut JNIEnv, object: &JObject) -> Result<T, ()>;
}


pub struct BoolField {
    meta_data: FieldMetaData,
}

impl BoolField {
    pub unsafe fn new(jre: *mut JNIEnv, name: &str, class: JClass) -> Result<BoolField, ()> {
        return Ok(BoolField {
            meta_data: try!(FieldMetaData::new(jre, name, "Z", class))
        })
    }
}

impl FieldGetter<bool> for BoolField {
    unsafe fn get(&self, jre: *mut JNIEnv, object: &JObject) -> Result<bool, ()> {
        return Ok(object.get_boolean_field(jre, self.meta_data.field_id));
    }    
}

pub trait ObjectWrapper<Object> : Sized {

    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<Object, ()> where Self: Sized;
    
    unsafe fn destroy_object_jlong(ptr: jlong) where Self: Sized {
        free_struct::<Object>(ptr);
    }
    
}

#[derive(Debug, Clone)]
pub struct EnumWrapper<T: Clone> {
    fully_qualified_class_name: String,
    ordinal_method_id: jmethodID,
    id_map: HashMap<jint, T>,
}

impl<T: Clone> EnumWrapper<T> {
   
    pub unsafe fn new(jre: *mut JNIEnv, enum_class: JClass, fqcn: String, mapping: &HashMap<&str, T>) -> Result<EnumWrapper<T>, ()>{
        let type_sig = format!("L{};", fqcn);
        let ordinal_method_id = try!(enum_class.get_method_id(jre, 
            "ordinal".to_string(),
            "()I".to_string()));
        let mut id_map: HashMap<jint, T> = HashMap::new();
        for (value_name, value) in mapping.iter() {
            let field_id = try!(enum_class.get_static_field_id(jre, value_name.clone(), type_sig.clone())); // TODO use reference instead of clone?
            let field_value = enum_class.get_static_object_field(jre, field_id);
            let id = field_value.call_int_method(jre, ordinal_method_id);
            id_map.insert(id, value.clone());
        }
        return Ok(EnumWrapper {
            fully_qualified_class_name: fqcn,
            ordinal_method_id: ordinal_method_id,
            id_map: id_map
        })
    }
    
    pub unsafe fn load(jre: *mut JNIEnv, package: &str, name: &str, mapping: &HashMap<&str, T>) -> Result<EnumWrapper<T>, ()> {
        return EnumWrapper::<T>::new(
            jre,
            try!(JClass::load(jre, package, name)),
            mangled_class_name(package, name),
            mapping);
    }
    
    pub unsafe fn cast(&self, jre: *mut JNIEnv, object: JObject) -> Result<T, ()> {
        let id = object.call_int_method(jre, self.ordinal_method_id);
        // TODO simplify
        return match self.id_map.get(&id) {
            Some(x) => Ok(x.clone()),
            None => Err(()),
        }
    }

    pub fn type_signature(&self) -> String {
        let mut result = "L".to_string();
        result.push_str(&self.fully_qualified_class_name);
        result.push_str(";");
        return result;
    }
}

pub struct EnumField<T: Clone> {
    meta_data: FieldMetaData,
    enum_wrapper: Rc<EnumWrapper<T>>,
}

impl<T: Clone> EnumField<T> {
    pub unsafe fn new(jre: *mut JNIEnv, enum_wrapper: Rc<EnumWrapper<T>>, name: &str, class: JClass) -> Result<EnumField<T>, ()> {
        let s = enum_wrapper.type_signature().to_owned();
        return Ok(EnumField {
            meta_data: try!(FieldMetaData::new(jre, name, &s[..], class)),
            enum_wrapper: enum_wrapper,
        })
    }
}

impl<T: Clone> FieldGetter<T> for EnumField<T> {
    unsafe fn get(&self, jre: *mut JNIEnv, object: &JObject) -> Result<T, ()> {
        // TODO why not just take equality on jobject (globalref)
        let value = object.get_object_field(jre, self.meta_data.field_id);
        return self.enum_wrapper.cast(jre, value);
    }    
}

// TODO automatically deduce Object from ObjectWrapper (type argument)
pub struct ObjectField<Object, Wrapper: ObjectWrapper<Object>> {
    meta_data: FieldMetaData,
    wrapper: Rc<Wrapper>,
    phantom: PhantomData<Object>
}

impl<Object, Wrapper: ObjectWrapper<Object>> ObjectField<Object, Wrapper> {
    // TODO ideally we could take the fqcn from JClass?
    pub unsafe fn new(jre: *mut JNIEnv, name: &str, class: JClass, package: &str, class_name: &str, wrapper: Rc<Wrapper>) -> Result<ObjectField<Object, Wrapper>, ()> {
        return Ok(ObjectField {
            meta_data: try!(FieldMetaData::new(jre, name, &JClass::type_signature(package, class_name), class)),
            wrapper: wrapper,
            phantom: PhantomData,
        })
    }
}

impl<Object, Wrapper: ObjectWrapper<Object>> FieldGetter<Object> for ObjectField<Object, Wrapper> {
    unsafe fn get(&self, jre: *mut JNIEnv, object: &JObject) -> Result<Object, ()> {
        let value = object.get_object_field(jre, self.meta_data.field_id);
        return self.wrapper.create_object(jre, value);
    }
}
