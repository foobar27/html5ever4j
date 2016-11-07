use std::str;

use std::io::{Write};
use std::default::Default;

use tendril::{StrTendril,TendrilSink};

use html5ever::driver::ParseOpts;
use html5ever::rcdom::{NodeEnum, ElementEnum, RcDom, Document, Doctype, Text, Comment, Element, Handle, AnnotationXml, Normal, Script, Template};
use html5ever::serialize::SerializeOpts;

use html5ever::{parse_document, serialize};

use html5ever_atoms::{LocalName, Namespace};

fn parse_string(input: String, opts: &ParseOpts) -> RcDom {
    return parse_document(RcDom::default(), opts.clone())
        .one(StrTendril::from_slice(&input).clone());
}

pub fn html2html(input: String, parse_opts: &ParseOpts, serialize_opts: &SerializeOpts) -> String {
    let dom = parse_string(input, parse_opts);
    let mut buf = Vec::<u8>::new();
    buf.write_all(b"<!DOCTYPE html>\n") // TODO make optional?
        .ok().expect("writing DOCTYPE failed");
    serialize(&mut buf, &dom.document, serialize_opts.clone())
        .ok().expect("serialization failed");
    match str::from_utf8(&*buf) {
        Ok(v) => return String::from(v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

pub struct Attribute {
    pub ns: String,
    pub key: String,
    pub value: String,
}

pub trait Callback {
    fn pre_order_visit(&self);
    fn set_doc_type(&self, name: String, public: String, system: String);
    fn create_text(&self, text: String);
    fn create_comment(&self, comment: String);
    fn create_normal_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>);
    fn create_script_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>, already_started: bool);
    fn create_template_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>);
    fn create_annotation_xml_element(&self, ns: &Namespace, tag: &LocalName, attributes: Vec<Attribute>, b: bool);
}

fn pre_visit<C: Callback>(node: &NodeEnum, callback: &C) {
    match *node {
        Document => {
            // skip
        },
        Doctype(ref name, ref public, ref system) => {
            // skip
        },
        Text(ref text) => {
            // skip
        },
        Comment(ref text) => {
            // skip
        },
        Element(ref name, ref element, ref attributes) => {
            callback.pre_order_visit();
        }
    }
}

fn post_visit<C: Callback>(node: &NodeEnum, callback: &C) {
    match *node {
        Document => {
            // skip
        },
        Doctype(ref name, ref public, ref system) => {
            callback.set_doc_type(name.to_string(), public.to_string(), system.to_string());
        },
        Text(ref text) => {
            callback.create_text(text.to_string());
        },
        Comment(ref text) => {
            callback.create_comment(text.to_string());
        },
        Element(ref name, ref element, ref attributes) => {
            let ref ns = name.ns;
            let ref tag = name.local;
            let mut attrs = Vec::<Attribute>::with_capacity(attributes.len());
            for attr in attributes.iter() {
                attrs.push(Attribute {
                    ns: attr.name.ns.to_string(),
                    key: attr.name.local.to_string(),
                    value: attr.value.to_string(),
                });
            }
            match *element {
                Normal => 
                    callback.create_normal_element(ns, tag, attrs),
                Script(already_started) =>
                    callback.create_script_element(ns, tag, attrs, already_started),
                Template(_) => {
                    // TODO argument ignored!
                    callback.create_template_element(ns, tag, attrs);
                },
                AnnotationXml(b) =>
                    callback.create_annotation_xml_element(ns, tag, attrs, b),
            }
        }
    }
}

fn parse_rec<C: Callback>(handle: Handle, callback: &C) {
    let node = handle.borrow();
    pre_visit(&node.node, callback);
    for child in node.children.iter() {
        parse_rec(child.clone(), callback);
    }
    post_visit(&node.node, callback);
}

pub fn parse<C: Callback>(input: String, parse_opts: &ParseOpts, callback: &C) {
    let dom = parse_string(input, parse_opts);
    // TODO errors via callback
    parse_rec(dom.document, callback);
}


