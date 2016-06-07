// #![crate_type = "dylib"]
//#![feature(const_fn)]

extern crate tendril;
extern crate html5ever;

extern crate libc;
extern crate jni_sys;

mod algorithms;
mod options;
mod native;
mod jni;
mod helper;

use std::result;
use std::str;
use std::mem;
use std::ptr;
use std::option;
use std::ptr::{drop_in_place,copy};
use std::fmt;
use std::fmt::{Formatter,Display};
use jni_sys::{jlong,jboolean,jstring,jfieldID,jobject,jclass,JNIEnv};
use libc::c_char;
use std::ffi::{CStr,CString};
use std::collections::HashMap;

use html5ever::driver::ParseOpts;
use html5ever::serialize::{SerializeOpts,TraversalScope};
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::interface::QuirksMode;

use jni::{FieldGetter,BoolField,EnumField,JObject,JClass,EnumWrapper,ObjectWrapper, box_to_jlong, free_struct, jstring_to_string, string_to_jstring};

use options::{Context,TokenizerOptionsWrapper,TreeBuilderOptionsWrapper,SerializeOptionsWrapper,ParseOptionsWrapper,FromContext,DebugString};


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createContext(
    jre: *mut JNIEnv, _: jclass) -> jlong {
    let context = Context::new(jre);
    return match context {
        Ok(context) => return box_to_jlong(context),
        Err(()) => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyContext(
jre: *mut JNIEnv, _: jclass, ptr: jlong) {
    free_struct::<Context>(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createTokenizerOptions(
    jre: *mut JNIEnv,
    _: jclass,
    context: jlong,
    object: jobject) -> jlong {
    return TokenizerOptionsWrapper::from_context_jlong(jre, context, object);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyTokenizerOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    TokenizerOptionsWrapper::destroy_object_jlong(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_tokenizerOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut TokenizerOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createTreeBuilderOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return TreeBuilderOptionsWrapper::from_context_jlong(jre, context, object);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyTreeBuilderOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    TreeBuilderOptionsWrapper::destroy_object_jlong(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_treeBuilderOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut TreeBuilderOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createParseOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return ParseOptionsWrapper::from_context_jlong(jre, context, object);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyParseOptions(
    jre: *mut JNIEnv, _: jclass, ptr: jlong) {
    ParseOptionsWrapper::destroy_object_jlong(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_parseOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut ParseOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createSerializeOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return SerializeOptionsWrapper::from_context_jlong(jre, context, object);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroySerializeOptions(
    jre: *mut JNIEnv, class: jclass, ptr: jlong) {
    SerializeOptionsWrapper::destroy_object_jlong(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_serializeOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut SerializeOpts);
    return string_to_jstring(jre, options.debug_string());
}

// #[no_mangle]
// pub unsafe extern fn Java_com_github_foobar27_html5ever4j_Native_html2html(
//     jre: *mut JNIEnv, class: jclass, input: jstring, parse_opts: jlong, serialize_opts: jlong) -> jstring {
    
//     let ref parse_opts = *(parse_opts as *mut ParseOpts);
//     let ref serialize_opts = *(serialize_opts as *mut SerializeOpts);
//     return helper::to_ptr(algorithm::html2html(
//         jstring_to_string(jre, input),
//         parse_opts,
//         serialize_opts));
// }


// TODO also allow to parse fragments?
// #[no_mangle]
// pub unsafe extern fn html2html(string: *const c_char,
//                  parse_opts: &ParseOpts,
//                                serialize_opts: &SerializeOpts)
//                                -> *const i8 {
// }
