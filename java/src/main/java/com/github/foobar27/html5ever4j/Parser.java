package com.github.foobar27.html5ever4j;

import java.util.*;

class Parser<N> {

    private final ParseOptions parseOptions;

    Parser(ParseOptions parseOptions) {
        this.parseOptions = parseOptions;
    }

    public void parse(String inputHtml, Sink<N> sink) {
        Native.getInstance().parse(inputHtml, parseOptions, new CallBack(sink));
    }

    static class CallBack {
        // node types
        private static final int NODE_TYPE_DOCUMENT = 0;
        private static final int NODE_TYPE_DOC_TYPE = 1;
        private static final int NODE_TYPE_TEXT = 2;
        private static final int NODE_TYPE_COMMENT = 3;

        // element-specific node types
        private static final int NODE_TYPE_ELEMENT_NORMAL = 4;
        private static final int NODE_TYPE_ELEMENT_SCRIPT_FIRST = 5;
        private static final int NODE_TYPE_ELEMENT_SCRIPT_ALREADY_STARTED = 6;
        private static final int NODE_TYPE_TEMPLATE = 7; // TODO where to store handle?
        private static final int NODE_TYPE_ANNOTATION_XML_FALSE = 8; // TODO bad naming!
        private static final int NODE_TYPE_ANNOTATION_XML_TRUE = 9; // TODO bad naming!

        final Sink<?> sink;

        CallBack(Sink<?> sink) {
            this.sink = sink;
        }

        // The type of a node.
        int type;

        // The name and attributes of an ELEMENT.
        String name;
        String[] keys;
        String[] values;

        // The text of a TEXT or COMMENT.
        String text;

        void setDocType(String name, String _public, String system) {
            sink.setDocType(name, _public, system); // TODO result ignored
        }

        void createText(String text) {
            sink.createText(text); // TODO result ignored
        }

        void createComment(String comment) {
            sink.createComment(comment); // TODO result ignored
        }

        void createNormalElement(String ns, String tagName, String[] attributes) {
            sink.createNormalElement(ns, tagName, parseAttributes(attributes));
        }

        void createScriptElement(String ns, String tagName, String[] attributes, boolean alreadyStarted) {
            sink.createScriptElement(ns, tagName, parseAttributes(attributes), alreadyStarted);
        }

        void createTemplateElement(String ns, String tagName, String[] attributes) {
            sink.createTemplateElement(ns, tagName, parseAttributes(attributes));
        }

        void createAnnotationXmlElement(String ns, String tagName, String[] attributes, boolean flag) { // TODO rename 'flag'
            sink.createAnnotationXmlElement(ns, tagName, parseAttributes(attributes), flag);
        }

        private static List<Sink.Attribute> parseAttributes(String[] xs) {
            assert (xs.length % 3 == 0);
            if (xs.length == 0) {
                return Collections.emptyList();
            }
            List<Sink.Attribute> attributes = new ArrayList<>();
            for (int i = 0; i < xs.length; i += 3) {
                attributes.add(new Sink.Attribute(xs[i], xs[i + 1], xs[i + 2]));
            }
            return attributes;
        }

    }

}
