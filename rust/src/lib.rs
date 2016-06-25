// #![crate_type = "dylib"]

extern crate tendril;
extern crate html5ever;

extern crate libc;
extern crate jni_sys;

mod algorithms;
mod options;
mod jni;
mod callbacks;
mod helper;

use jni_sys::{jlong,jstring,jobject,jclass,JNIEnv};

use html5ever::driver::ParseOpts;
use html5ever::serialize::SerializeOpts;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;

use jni::{ObjectWrapper, box_to_jlong, free_struct, string_to_jstring, jstring_to_string};

use options::{Context,TokenizerOptionsWrapper,TreeBuilderOptionsWrapper,SerializeOptionsWrapper,ParseOptionsWrapper,FromContext,DebugString};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createContext(
    jre: *mut JNIEnv, _: jclass) -> jlong {
    let context = Context::new(jre);
    return match context {
        Ok(context) => return box_to_jlong(context),
        Err(()) => 0
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyContext(
_: *mut JNIEnv, _: jclass, ptr: jlong) {
    free_struct::<Context>(ptr);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createTokenizerOptions(
    jre: *mut JNIEnv,
    _: jclass,
    context: jlong,
    object: jobject) -> jlong {
    return TokenizerOptionsWrapper::from_context_jlong(jre, context, object);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyTokenizerOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    TokenizerOptionsWrapper::destroy_object_jlong(ptr);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_tokenizerOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut TokenizerOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createTreeBuilderOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return TreeBuilderOptionsWrapper::from_context_jlong(jre, context, object);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyTreeBuilderOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    TreeBuilderOptionsWrapper::destroy_object_jlong(ptr);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_treeBuilderOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut TreeBuilderOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createParseOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return ParseOptionsWrapper::from_context_jlong(jre, context, object);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroyParseOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    ParseOptionsWrapper::destroy_object_jlong(ptr);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_parseOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut ParseOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_createSerializeOptions(
    jre: *mut JNIEnv, _: jclass, context: jlong, object: jobject) -> jlong {
    return SerializeOptionsWrapper::from_context_jlong(jre, context, object);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_destroySerializeOptions(
    _: *mut JNIEnv, _: jclass, ptr: jlong) {
    SerializeOptionsWrapper::destroy_object_jlong(ptr);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_foobar27_html5ever4j_Native_serializeOptionsToString(
    jre: *mut JNIEnv, _: jclass, options: jlong) -> jstring {
    let ref options = *(options as *mut SerializeOpts);
    return string_to_jstring(jre, options.debug_string());
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_com_github_foobar27_html5ever4j_Native_html2html(
    jre: *mut JNIEnv, class: jclass, input: jstring, parse_opts: jlong, serialize_opts: jlong) -> jstring {
    
    let ref parse_opts = *(parse_opts as *mut ParseOpts);
    let ref serialize_opts = *(serialize_opts as *mut SerializeOpts);
    return string_to_jstring(
        jre,
        algorithms::html2html(
            jstring_to_string(jre, input),
            parse_opts,
            serialize_opts));
}


// TODO also allow to parse fragments?
// #[allow(non_snake_case)]
// #[no_mangle]
// pub unsafe extern fn html2html(string: *const c_char,
//                  parse_opts: &ParseOpts,
//                                serialize_opts: &SerializeOpts)
//                                -> *const i8 {
// }
