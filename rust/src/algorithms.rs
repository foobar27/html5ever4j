use std::str;

use std::io::{Write};
use std::default::Default;

use tendril::{StrTendril,TendrilSink};

use html5ever::driver::ParseOpts;
use html5ever::rcdom::RcDom;
use html5ever::serialize::SerializeOpts;

use html5ever::{parse_document, serialize};

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
    fn set_doc_type(&self, name: String, public: String, system: String);
    fn create_text(&self, text: String);
    fn create_comment(&self, comment: String);
    fn create_normal_element(&self, ns: String, tag_name: String, attributes: Vec<Attribute>);
}

// TODO move to JNI, factor out 'CallBack' trait 

pub struct Attribute {
    pub ns: String,
    pub key: String,
    pub value: String,
}

