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

use algorithms;
use jni;
use jni::{FieldGetter,BoolField,EnumField,JObject,JClass,EnumWrapper,ObjectWrapper, box_to_jlong, free_struct, jstring_to_string, string_to_jstring};

static PACKAGE: &'static str = "com.github.foobar27.html5ever4j";

pub struct TokenizerOptionsWrapper {
    exact_errors: jni::BoolField,
    discard_bom: jni::BoolField,
}

pub struct TreeBuilderOptionsWrapper {
    exact_errors: jni::BoolField,
    scripting_enabled: jni::BoolField,
    iframe_srcdoc: jni::BoolField,
    drop_doctype: jni::BoolField,
    quirks_mode: jni::EnumField<QuirksMode>,
}

pub struct SerializeOptionsWrapper {
    scripting_enabled: jni::BoolField,
    traversal_scope: jni::EnumField<TraversalScope>,
}

pub struct ParseOptionsWrapper {
    tokenizer_opts: jni::ObjectField<TokenizerOpts, TokenizerOptionsWrapper>,
    tree_builder_opts: jni::ObjectField<TreeBuilderOpts, TreeBuilderOptionsWrapper>,
}

pub struct Context {
    tokenizer_options_wrapper: TokenizerOptionsWrapper,
    tree_builder_options_wrapper: TreeBuilderOptionsWrapper,
    serialize_options_wrapper: SerializeOptionsWrapper,
    parse_options_wrapper: ParseOptionsWrapper,
}

// TODO remove 'This' argument!
pub trait FromContext<Object, This: FromContext<Object, This>> {

    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<Object, ()>;
    
    unsafe fn from_context_jlong(jre: *mut JNIEnv, context: jlong, object: jobject) -> jlong {
        let ref context = *(context as *mut Context);
        let object = JObject::new_borrowed(object);
        match This::from_context(jre, context, object) {
            Ok(object) => box_to_jlong(object),
            Err(()) => 0
        }
    }
}

pub trait DebugString {
    fn debug_string(&self) -> String;
    //fn fmt(&self, f: &mut Formatter) -> fmt::Result;
}

impl DebugString for TokenizerOpts {
    fn debug_string(&self) -> String {
        return format!("TokenizerOpts[exact_errors={},discard_bom={}]",
                       self.exact_errors,
                       self.discard_bom);
    }
    // fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    //     fmt.debug_struct("TokenizerOpts")
    //         .field("exact_errors", &self.exact_errors)
    //         .field("discard_bom", &self.discard_bom)
    //         .finish()
    // }
}


impl TokenizerOptionsWrapper {
    unsafe fn new(jre: *mut JNIEnv, class: JClass) -> Result<TokenizerOptionsWrapper, ()> {
        return Ok(TokenizerOptionsWrapper {
            exact_errors: try!(jni::BoolField::new(jre, "reportExactErrors", class.clone())),
            discard_bom: try!(jni::BoolField::new(jre, "discardBom", class.clone())),
        })
    }

    unsafe fn load(jre: *mut JNIEnv) -> Result<TokenizerOptionsWrapper, ()> {
        return TokenizerOptionsWrapper::new(jre, try!(JClass::load(jre, PACKAGE, "TokenizerOptions")));
    }
}

impl ObjectWrapper<TokenizerOpts> for TokenizerOptionsWrapper {

    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<TokenizerOpts, ()> {
        return Ok(TokenizerOpts {
            exact_errors: try!(self.exact_errors.get(jre, &object)),
            discard_bom: try!(self.discard_bom.get(jre, &object)),
            profile: false,
            initial_state: None,
            last_start_tag_name: None,
        })
    }
}

impl FromContext<TokenizerOpts, TokenizerOptionsWrapper> for TokenizerOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<TokenizerOpts, ()> {
        return context.tokenizer_options_wrapper.create_object(jre, object);
    }
}

impl DebugString for QuirksMode {
    fn debug_string(&self) -> String {
        match self {
            &QuirksMode::Quirks => "Quirks",
            &QuirksMode::LimitedQuirks => "LimitedQuirks",
            &QuirksMode::NoQuirks => "NoQuirks",
        }.to_string()
    }
}

impl DebugString for TreeBuilderOpts {
    fn debug_string(&self) -> String {
        return format!("TreeBuilderOpts[exact_errors={},scripting_enabled={},iframe_srcdoc={},drop_doctype={},quirks_mode={}]",
                       self.exact_errors,
                       self.scripting_enabled,
                       self.iframe_srcdoc,
                       self.drop_doctype,
                       self.quirks_mode.debug_string());
    }
}

impl TreeBuilderOptionsWrapper {

    unsafe fn new(jre: *mut JNIEnv, class: JClass, quirks_mode_wrapper: &EnumWrapper<QuirksMode>) -> Result<TreeBuilderOptionsWrapper, ()> {
        return Ok(TreeBuilderOptionsWrapper {
            exact_errors: try!(jni::BoolField::new(jre, "reportExactErrors", class.clone())),
            scripting_enabled: try!(jni::BoolField::new(jre, "scriptingEnabled", class.clone())),
            iframe_srcdoc: try!(jni::BoolField::new(jre, "iframeSrcdoc", class.clone())),
            drop_doctype: try!(jni::BoolField::new(jre, "dropDoctype", class.clone())),
            quirks_mode: try!(jni::EnumField::new(jre, quirks_mode_wrapper, "quirksMode", class.clone()))
        })
    }

    unsafe fn load(jre: *mut JNIEnv, quirks_mode_wrapper: &EnumWrapper<QuirksMode>) -> Result<TreeBuilderOptionsWrapper, ()> {
        return TreeBuilderOptionsWrapper::new(
            jre,
            try!(JClass::load(jre, PACKAGE, "TreeBuilderOptions")),
            quirks_mode_wrapper);
    }

}

impl ObjectWrapper<TreeBuilderOpts> for TreeBuilderOptionsWrapper {
    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<TreeBuilderOpts, ()> {
        return Ok(TreeBuilderOpts {
            exact_errors: try!(self.exact_errors.get(jre, &object)),
            scripting_enabled: try!(self.scripting_enabled.get(jre, &object)),
            iframe_srcdoc: try!(self.iframe_srcdoc.get(jre, &object)),
            drop_doctype: try!(self.drop_doctype.get(jre, &object)),
            quirks_mode: QuirksMode::NoQuirks, // TODO
            ignore_missing_rules: false, // TODO verify
        })
    }
}

impl FromContext<TreeBuilderOpts, TreeBuilderOptionsWrapper> for TreeBuilderOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<TreeBuilderOpts, ()> {
        return context.tree_builder_options_wrapper.create_object(jre, object);
    }
}

impl SerializeOptionsWrapper {
    unsafe fn new(jre: *mut JNIEnv, class: JClass, traversal_scope_wrapper: &EnumWrapper<TraversalScope>) -> Result<SerializeOptionsWrapper, ()> {
        return Ok(SerializeOptionsWrapper {
            scripting_enabled: try!(jni::BoolField::new(jre, "scriptingEnabled", class.clone())),
            traversal_scope: try!(jni::EnumField::new(jre, traversal_scope_wrapper, "traversalScope", class.clone()))
        });
    }

    unsafe fn load(jre: *mut JNIEnv, traversal_scope_wrapper: &EnumWrapper<TraversalScope>) -> Result<SerializeOptionsWrapper, ()> {
        let class = try!(JClass::load(jre, PACKAGE, "SerializeOptions"));
        return SerializeOptionsWrapper::new(jre, class, traversal_scope_wrapper);
    }

}

impl ObjectWrapper<SerializeOpts> for SerializeOptionsWrapper {
    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<SerializeOpts, ()> {
        return Ok(SerializeOpts {
            scripting_enabled: try!(self.scripting_enabled.get(jre, &object)),
            traversal_scope: TraversalScope::ChildrenOnly,
        })
    }
}

impl FromContext<SerializeOpts, SerializeOptionsWrapper> for SerializeOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<SerializeOpts, ()> {
        return context.serialize_options_wrapper.create_object(jre, object);
    }
}

impl Context {

    pub unsafe fn new(jre: *mut JNIEnv) -> Result<Context, ()> {
        let mut quirks_mapping = HashMap::<&str, QuirksMode>::new();
        quirks_mapping.insert("QUIRKS", QuirksMode::Quirks);
        quirks_mapping.insert("LIMITED_QUIRKS", QuirksMode::LimitedQuirks);
        quirks_mapping.insert("NO_QUIRKS", QuirksMode::NoQuirks);
        let quirks_mode_wrapper = try!(EnumWrapper::<QuirksMode>::load(jre, PACKAGE, "QuirksMode", &quirks_mapping));

        let mut traversal_scope_mapping = HashMap::<&str, TraversalScope>::new();
        traversal_scope_mapping.insert("INCLUDE_NODE", TraversalScope::IncludeNode);
        traversal_scope_mapping.insert("CHILDREN_ONLY", TraversalScope::ChildrenOnly);
        let traversal_scope_wrapper = try!(EnumWrapper::<TraversalScope>::load(jre, PACKAGE, "SerializeOptions$TraversalScope", &traversal_scope_mapping));

        return Ok(Context {
            tokenizer_options_wrapper: try!(TokenizerOptionsWrapper::load(jre)),
            tree_builder_options_wrapper: try!(TreeBuilderOptionsWrapper::load(jre, &quirks_mode_wrapper)),
            serialize_options_wrapper: try!(SerializeOptionsWrapper::load(jre, &traversal_scope_wrapper)),
        });
    }
    
}

