package com.github.foobar27.html5ever4j;

import java.util.*;

class Parser {

    //private final Sink<N> sink;

    // TODO also consider text node, comments etc

    static class Attribute {
        String namespace;
        String key;
        String value;
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

        //final Sink<N> sink;

        // The type of a node.
        int type;

        // The name and attributes of an ELEMENT.
        String name;
        String[] keys;
        String[] values;

        // The text of a TEXT or COMMENT.
        String text;

        void setDocType(String name, String _public, String system) {
            System.out.println(String.format("setDocType(%s,%s,%s)", name, _public, system));
            //return sink.setDocType(name, _public, system);
        }

        void createText(String text) {
            System.out.println(String.format("createText(%s)", text));
            //return sink.createText(text);
        }

        void createComment(String comment) {
            System.out.println(String.format("createComment(%s)", comment));
            //return sink.createComment(comment);
        }

        void createNormalElement(String ns, String tagName, String[] attributes) {
            assert(attributes.length % 3 == 0);
            System.out.println(String.format("createNormalElement(%s, %s, %s)",
                    ns,
                    tagName,
                    Arrays.asList(attributes)));
            //sink.createElement(ns, tagName, )
        }

        void createScriptElement(String ns, String tagName, String[] attributes, boolean alreadyStarted) {
            assert(attributes.length % 3 == 0);
            System.out.println(String.format("createScriptElement(%s, %s, %s, %b)",
                    ns,
                    tagName,
                    Arrays.asList(attributes),
                    alreadyStarted));
            //sink.createElement(ns, tagName, )
        }

        void createTemplateElement(String ns, String tagName, String[] attributes) {
            assert(attributes.length % 3 == 0);
            System.out.println(String.format("createTemplateElement(%s, %s, %s)",
                    ns,
                    tagName,
                    Arrays.asList(attributes)));
            //sink.createElement(ns, tagName, )
        }

        void createAnnotationXmlElement(String ns, String tagName, String[] attributes, boolean flag) {
            assert(attributes.length % 3 == 0);
            System.out.println(String.format("createAnnotationXmlElement(%s, %s, %s, %b)",
                    ns,
                    tagName,
                    Arrays.asList(attributes),
                    flag));
            //sink.createElement(ns, tagName, )
        }

        private static List<Map.Entry<String, String>> combineAttributes(String[] keys, String[] values) {
            assert(keys.length == values.length);
            if (keys.length == 0) {
                return Collections.emptyList();
            }
            List<Map.Entry<String, String>> attributes = new ArrayList<>();
            for (int i=0; i<keys.length; ++i) {
                attributes.add(new AbstractMap.SimpleEntry<>(keys[i], values[i]));
            }
            return attributes;
        }

        void goUp() {

        }

    }

}
