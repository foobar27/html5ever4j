use std::ptr;
use std::rc::Rc;
use jni_sys::{jlong,jint,jstring,jclass,jobject,jmethodID,jfieldID,jobjectArray,jintArray,JNIEnv};
use std::collections::HashMap;
use std::marker::PhantomData;

use helper::{to_ptr,to_string};

// TODO verify exception ocurred???
macro_rules! jni {
    ($jre:expr, $method_name:ident, $( $arg:expr ),*) =>
        (((**$jre).$method_name)($jre, $( $arg ),*);)
}

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
    let chars = jni!(jre, GetStringUTFChars, s, ptr::null_mut());
    let result = to_string(chars);
    jni!(jre, ReleaseStringUTFChars, s, chars); 
    return result
}

pub unsafe fn string_to_jstring<S>(jre: *mut JNIEnv, s: S) -> jstring
    where S: Into<String> {
    return jni!(jre, NewStringUTF, to_ptr(s));
}

pub unsafe fn jobject_vec_to_jobjectArray(jre: *mut JNIEnv, items: &Vec<jobject>, item_class: JClass) -> Result<jobjectArray, ()> {
    let jni_result = jni!(jre, NewObjectArray, items.len() as i32, item_class.class, ptr::null_mut());
    if jni_result.is_null() {
        return Err(())
    }
    for (index, item) in items.iter().enumerate() {
        jni!(jre, SetObjectArrayElement, jni_result, index as i32, *item);
    }
    return Ok(jni_result);
}

pub unsafe fn i32_vec_to_jintArray(jre: *mut JNIEnv, items: &Vec<i32>) -> Result<jintArray, ()> {
    let jni_result = jni!(jre, NewIntArray, items.len() as i32);
    if jni_result.is_null() {
        return Err(());
    }
    jni!(jre, SetIntArrayRegion, jni_result, 0 as i32, items.len() as i32, items.as_ptr());
    return Ok(jni_result);
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
    pub object: jobject, // TODO make private if varargs is properly supported
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
        return jni!(jre, GetBooleanField, self.object, id) > 0
    }

    // TODO make generic
    unsafe fn get_object_field(&self, jre: *mut JNIEnv, id: jfieldID) -> JObject {
        return JObject::new_owned(jni!(jre, GetObjectField, self.object, id))
    }

    // TODO make generic: .call_method<JObject>(id)?
    // TODO allow parameters
    pub unsafe fn call_object_method(&self, jre: *mut JNIEnv, method_id: jmethodID) -> JObject {
        return JObject::new_owned(
            jni!(jre, CallObjectMethod,
                 self.object,
                 method_id));
    }

    // TODO allow parameters
    pub unsafe fn call_int_method(&self, jre: *mut JNIEnv, id: jmethodID) -> jint {
        return jni!(jre, CallIntMethod, self.object, id);
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

    pub unsafe fn create_global_ref(&self, jre: *mut JNIEnv) -> Result<JClass, ()> {
        let class = jni!(jre, NewGlobalRef, self.class);
        if class.is_null() {
            return Err(())
        }
        return Ok(JClass::new_owned(class))
    }    

    pub fn type_signature(package: &str, name: &str) -> String {
        let mut result = String::from("L");
        result.push_str(&mangled_class_name(package, name));
        result.push_str(";");
        return result;
    }
    
    pub unsafe fn load(jre: *mut JNIEnv, package: &str, name: &str) -> Result<JClass, ()> {
        let class = jni!(jre, FindClass, to_ptr(mangled_class_name(package, name)));
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
        let result = jni!(jre, GetFieldID,
                          self.class,
                          to_ptr(name),
                          to_ptr(sig));
        return jfield_id_result(result);
    }
   
    pub unsafe fn get_method_id(&self, jre: *mut JNIEnv, name: &str, signature: &str) -> Result<jmethodID, ()> {
        let result = jni!(jre, GetMethodID,
                          self.class,
                          to_ptr(name),
                          to_ptr(signature));
        return jmethod_id_result(result);
    }
    
    pub unsafe fn get_static_field_id(&self, jre: *mut JNIEnv, name: &str, signature: &str) -> Result<jfieldID, ()>{
        let result = jni!(jre, GetStaticFieldID,
                          self.class,
                          to_ptr(name),
                          to_ptr(signature));
        return jfield_id_result(result)
    }

    // TODO make generic
    pub unsafe fn get_static_object_field(&self, jre: *mut JNIEnv, id: jfieldID) -> JObject {
        return JObject::new_owned(
            jni!(jre, GetStaticObjectField, 
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
   
    pub unsafe fn new(jre: *mut JNIEnv, enum_class: JClass, fqcn: &str, mapping: &HashMap<&str, T>) -> Result<EnumWrapper<T>, ()>{
        let type_sig = format!("L{};", fqcn);
        let ordinal_method_id = try!(enum_class.get_method_id(jre, "ordinal", "()I"));
        let mut id_map: HashMap<jint, T> = HashMap::new();
        for (value_name, value) in mapping.iter() {
            let field_id = try!(enum_class.get_static_field_id(jre, value_name, &type_sig));
            let field_value = enum_class.get_static_object_field(jre, field_id);
            let id = field_value.call_int_method(jre, ordinal_method_id);
            id_map.insert(id, value.clone());
        }
        return Ok(EnumWrapper {
            fully_qualified_class_name: fqcn.to_string(),
            ordinal_method_id: ordinal_method_id,
            id_map: id_map
        })
    }
    
    pub unsafe fn load(jre: *mut JNIEnv, package: &str, name: &str, mapping: &HashMap<&str, T>) -> Result<EnumWrapper<T>, ()> {
        return EnumWrapper::<T>::new(
            jre,
            try!(JClass::load(jre, package, name)),
            &mangled_class_name(package, name),
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
        let mut result = String::from("L");
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
