use std::rc::Rc;
use jni_sys::{jlong,jobject,JNIEnv};

use html5ever::driver::ParseOpts;
use html5ever::serialize::SerializeOpts;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;

use jni::{JObject,ObjectWrapper, box_to_jlong};

use options::{TokenizerOptionsWrapper,TreeBuilderOptionsWrapper,SerializeOptionsWrapper,ParseOptionsWrapper,create_quirks_mode_wrapper,create_traversal_scope_wrapper};

pub struct Context {
    tokenizer_options_wrapper: Rc<TokenizerOptionsWrapper>,
    tree_builder_options_wrapper: Rc<TreeBuilderOptionsWrapper>,
    serialize_options_wrapper: Rc<SerializeOptionsWrapper>,
    parse_options_wrapper: Rc<ParseOptionsWrapper>,
}

// TODO rename to OptionsFromContext
pub trait FromContext<Object> {

    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<Object, ()>;
    
    unsafe fn from_context_jlong(jre: *mut JNIEnv, context: jlong, object: jobject) -> jlong {
        let ref context = *(context as *mut Context);
        let object = JObject::new_borrowed(object);
        match Self::from_context(jre, context, object) {
            Ok(object) => box_to_jlong(object),
            Err(()) => 0
        }
    }
}

impl FromContext<TokenizerOpts> for TokenizerOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<TokenizerOpts, ()> {
        return context.tokenizer_options_wrapper.create_object(jre, object);
    }
}

impl FromContext<TreeBuilderOpts> for TreeBuilderOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<TreeBuilderOpts, ()> {
        return context.tree_builder_options_wrapper.create_object(jre, object);
    }
}

impl FromContext<SerializeOpts> for SerializeOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<SerializeOpts, ()> {
        return context.serialize_options_wrapper.create_object(jre, object);
    }
}

impl FromContext<ParseOpts> for ParseOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<ParseOpts, ()> {
     return context.parse_options_wrapper.create_object(jre, object);   
    }
}

impl Context {

    pub unsafe fn new(jre: *mut JNIEnv) -> Result<Context, ()> {
        let quirks_mode_wrapper = Rc::new(try!(create_quirks_mode_wrapper(jre)));
        let traversal_scope_wrapper = Rc::new(try!(create_traversal_scope_wrapper(jre)));

        let tokenizer_options_wrapper = Rc::new(try!(TokenizerOptionsWrapper::load(jre)));
        let tree_builder_options_wrapper = Rc::new(try!(TreeBuilderOptionsWrapper::load(jre, quirks_mode_wrapper)));

        return Ok(Context {
            tokenizer_options_wrapper: tokenizer_options_wrapper.clone(),
            tree_builder_options_wrapper: tree_builder_options_wrapper.clone(),
            serialize_options_wrapper: Rc::new(try!(SerializeOptionsWrapper::load(jre, traversal_scope_wrapper))),
            parse_options_wrapper: Rc::new(try!(ParseOptionsWrapper::load(jre, tokenizer_options_wrapper, tree_builder_options_wrapper))),
        });
    }
    
}

