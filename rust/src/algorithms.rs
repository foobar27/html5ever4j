use std::str;

use std::io::{Write};
use std::default::Default;

use tendril::{StrTendril,TendrilSink};

use html5ever::driver::ParseOpts;
use html5ever::rcdom::{NodeEnum, ElementEnum, RcDom, Document, Doctype, Text, Comment, Element, Handle, AnnotationXml, Normal, Script, Template};
use html5ever::serialize::SerializeOpts;

use html5ever::{parse_document, serialize};

use html5ever_atoms::QualName;
use html5ever::tokenizer::Attribute;

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

pub trait Callback {
    fn pre_order_visit(&self);
    fn set_doc_type(&self, name: String, public: String, system: String);
    fn create_text(&self, text: String);
    fn create_comment(&self, comment: String);
    fn create_normal_element(&self, tag: &QualName, attributes: &Vec<Attribute>);
    fn create_script_element(&self, tag: &QualName, attributes: &Vec<Attribute>, already_started: bool);
    fn create_template_element(&self, tag: &QualName, attributes: &Vec<Attribute>);
    fn create_annotation_xml_element(&self, tag: &QualName, attributes: &Vec<Attribute>, b: bool);
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
            match *element {
                Normal => 
                    callback.create_normal_element(name, attributes),
                Script(already_started) =>
                    callback.create_script_element(name, attributes, already_started),
                Template(_) => {
                    // TODO argument ignored!
                    callback.create_template_element(name, attributes);
                },
                AnnotationXml(b) =>
                    callback.create_annotation_xml_element(name, attributes, b),
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


