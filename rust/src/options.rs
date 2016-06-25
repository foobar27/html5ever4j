use std::rc::Rc;
use std::str;
use jni_sys::{jlong,jobject,JNIEnv};
use std::collections::HashMap;

use html5ever::driver::ParseOpts;
use html5ever::serialize::{SerializeOpts,TraversalScope};
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::interface::QuirksMode;

use jni;
use jni::{FieldGetter,JObject,JClass,EnumWrapper,ObjectWrapper, box_to_jlong};

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
    tokenizer_options_wrapper: Rc<TokenizerOptionsWrapper>,
    tree_builder_options_wrapper: Rc<TreeBuilderOptionsWrapper>,
    serialize_options_wrapper: Rc<SerializeOptionsWrapper>,
    parse_options_wrapper: Rc<ParseOptionsWrapper>,
}

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

pub trait DebugString {
    fn debug_string(&self) -> String;
}

impl DebugString for TokenizerOpts {
    fn debug_string(&self) -> String {
        return format!("TokenizerOpts[exact_errors={},discard_bom={}]",
                       self.exact_errors,
                       self.discard_bom);
    }
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

impl FromContext<TokenizerOpts> for TokenizerOptionsWrapper {
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

impl DebugString for TraversalScope {
    fn debug_string(&self) -> String {
        match self {
            &TraversalScope::IncludeNode => "IncludeNode",
            &TraversalScope::ChildrenOnly => "ChildrenOnly",
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

    unsafe fn new(jre: *mut JNIEnv, class: JClass, quirks_mode_wrapper: Rc<EnumWrapper<QuirksMode>>) -> Result<TreeBuilderOptionsWrapper, ()> {
        return Ok(TreeBuilderOptionsWrapper {
            exact_errors: try!(jni::BoolField::new(jre, "reportExactErrors", class.clone())),
            scripting_enabled: try!(jni::BoolField::new(jre, "scriptingEnabled", class.clone())),
            iframe_srcdoc: try!(jni::BoolField::new(jre, "iframeSrcdoc", class.clone())),
            drop_doctype: try!(jni::BoolField::new(jre, "dropDoctype", class.clone())),
            quirks_mode: try!(jni::EnumField::new(jre, quirks_mode_wrapper, "quirksMode", class.clone()))
        })
    }

    unsafe fn load(jre: *mut JNIEnv, quirks_mode_wrapper: Rc<EnumWrapper<QuirksMode>>) -> Result<TreeBuilderOptionsWrapper, ()> {
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
            quirks_mode: try!(self.quirks_mode.get(jre, &object)),
            ignore_missing_rules: false, // TODO verify
        })
    }
}

impl FromContext<TreeBuilderOpts> for TreeBuilderOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<TreeBuilderOpts, ()> {
        return context.tree_builder_options_wrapper.create_object(jre, object);
    }
}

impl DebugString for SerializeOpts {
    fn debug_string(&self) -> String {
        return format!("SerializeOpts[scripting_enabled={},traversal_scope={}]",
                       self.scripting_enabled,
                       self.traversal_scope.debug_string());
    }
}

impl SerializeOptionsWrapper {
    unsafe fn new(jre: *mut JNIEnv, class: JClass, traversal_scope_wrapper: Rc<EnumWrapper<TraversalScope>>) -> Result<SerializeOptionsWrapper, ()> {
        return Ok(SerializeOptionsWrapper {
            scripting_enabled: try!(jni::BoolField::new(jre, "scriptingEnabled", class.clone())),
            traversal_scope: try!(jni::EnumField::new(jre, traversal_scope_wrapper, "traversalScope", class.clone()))
        });
    }

    unsafe fn load(jre: *mut JNIEnv, traversal_scope_wrapper: Rc<EnumWrapper<TraversalScope>>) -> Result<SerializeOptionsWrapper, ()> {
        let class = try!(JClass::load(jre, PACKAGE, "SerializeOptions"));
        return SerializeOptionsWrapper::new(jre, class, traversal_scope_wrapper);
    }

}

impl ObjectWrapper<SerializeOpts> for SerializeOptionsWrapper {
    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<SerializeOpts, ()> {
        return Ok(SerializeOpts {
            scripting_enabled: try!(self.scripting_enabled.get(jre, &object)),
            traversal_scope: try!(self.traversal_scope.get(jre, &object)),
        })
    }
}

impl FromContext<SerializeOpts> for SerializeOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<SerializeOpts, ()> {
        return context.serialize_options_wrapper.create_object(jre, object);
    }
}

impl DebugString for ParseOpts {
    fn debug_string(&self) -> String {
        return format!("ParseOpts[tokenizer={},tree_builder={}]",
                       self.tokenizer.debug_string(),
                       self.tree_builder.debug_string());
    }
}

impl ParseOptionsWrapper {
    unsafe fn new(jre: *mut JNIEnv, class: JClass, tokenizer_options: Rc<TokenizerOptionsWrapper>, tree_builder_options: Rc<TreeBuilderOptionsWrapper>) -> Result<ParseOptionsWrapper, ()> {
        return Ok(ParseOptionsWrapper {
            tokenizer_opts: try!(jni::ObjectField::new(
                jre,
                "tokenizerOptions",
                class.clone(),
                PACKAGE,
                "TokenizerOptions",
                tokenizer_options)),
            tree_builder_opts: try!(jni::ObjectField::new(
                jre,
                "treeBuilderOptions",
                class.clone(),
                PACKAGE,
                "TreeBuilderOptions",
                tree_builder_options)),
        });
    }
    unsafe fn load(jre: *mut JNIEnv, tokenizer_options: Rc<TokenizerOptionsWrapper>, tree_builder_options: Rc<TreeBuilderOptionsWrapper>) -> Result<ParseOptionsWrapper, ()> {
        let class = try!(JClass::load(jre, PACKAGE, "ParseOptions"));
        return ParseOptionsWrapper::new(jre, class, tokenizer_options, tree_builder_options);
    }
}

impl ObjectWrapper<ParseOpts> for ParseOptionsWrapper {
    unsafe fn create_object(&self, jre: *mut JNIEnv, object: JObject) -> Result<ParseOpts, ()> {
        return Ok(ParseOpts {
            tokenizer: try!(self.tokenizer_opts.get(jre, &object)),
            tree_builder: try!(self.tree_builder_opts.get(jre, &object)),
        });
    }
}

impl FromContext<ParseOpts> for ParseOptionsWrapper {
    unsafe fn from_context(jre: *mut JNIEnv, context: &Context, object: JObject) -> Result<ParseOpts, ()> {
     return context.parse_options_wrapper.create_object(jre, object);   
    }
}

unsafe fn create_quirks_mode_wrapper(jre: *mut JNIEnv) -> Result<EnumWrapper<QuirksMode>, ()> {
    let mut quirks_mapping = HashMap::<&str, QuirksMode>::new();
    quirks_mapping.insert("QUIRKS", QuirksMode::Quirks);
    quirks_mapping.insert("LIMITED_QUIRKS", QuirksMode::LimitedQuirks);
    quirks_mapping.insert("NO_QUIRKS", QuirksMode::NoQuirks);
    return EnumWrapper::<QuirksMode>::load(jre, PACKAGE, "QuirksMode", &quirks_mapping);
}

unsafe fn create_traversal_scope_wrapper(jre: *mut JNIEnv) -> Result<EnumWrapper<TraversalScope>, ()> {
    let mut traversal_scope_mapping = HashMap::<&str, TraversalScope>::new();
    traversal_scope_mapping.insert("INCLUDE_NODE", TraversalScope::IncludeNode);
    traversal_scope_mapping.insert("CHILDREN_ONLY", TraversalScope::ChildrenOnly);
    return EnumWrapper::<TraversalScope>::load(jre, PACKAGE, "SerializeOptions$TraversalScope", &traversal_scope_mapping);
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

