package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.atoms.LocalName;
import com.github.foobar27.html5ever4j.atoms.Namespace;

import java.util.*;

class Parser<N> {

    private final ParseOptions parseOptions;

    Parser(ParseOptions parseOptions) {
        this.parseOptions = parseOptions;
    }

    public void parse(String inputHtml, Visitor visitor) {
        Native.getInstance().parse(inputHtml, parseOptions, new CallBack(visitor));
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

        final Visitor visitor;

        CallBack(Visitor visitor) {
            this.visitor = visitor;
        }

        // The type of a node.
        int type;

        // The name and attributes of an ELEMENT.
        String name;
        String[] keys;
        String[] values;

        // The text of a TEXT or COMMENT.
        String text;

        void preOrderVisit() {
            visitor.preOrderVisit();
        }

        void setDocType(String name, String _public, String system) {
            visitor.setDocType(name, _public, system); // TODO result ignored
        }

        void createText(String text) {
            visitor.createText(text); // TODO result ignored
        }

        void createComment(String comment) {
            visitor.createComment(comment); // TODO result ignored
        }

        void createNormalElement(int nsId, String nsString, int tagId, String tagString, String[] attributes) {
            visitor.createNormalElement(
                    Namespace.getNamespace(nsId, nsString),
                    LocalName.getLocalName(tagId, tagString),
                    parseAttributes(attributes));
        }

        void createScriptElement(int nsId, String nsString, int tagId, String tagString, String[] attributes, boolean alreadyStarted) {
            visitor.createScriptElement(
                    Namespace.getNamespace(nsId, nsString),
                    LocalName.getLocalName(tagId, tagString),
                    parseAttributes(attributes), alreadyStarted);
        }

        void createTemplateElement(int nsId, String nsString, int tagId, String tagString, String[] attributes) {
            visitor.createTemplateElement(
                    Namespace.getNamespace(nsId, nsString),
                    LocalName.getLocalName(tagId, tagString),
                    parseAttributes(attributes));
        }

        void createAnnotationXmlElement(int nsId, String nsString, int tagId, String tagString, String[] attributes, boolean flag) { // TODO rename 'flag'
            visitor.createAnnotationXmlElement(
                    Namespace.getNamespace(nsId, nsString),
                    LocalName.getLocalName(tagId, tagString),
                    parseAttributes(attributes),
                    flag);
        }

        private static List<Visitor.Attribute> parseAttributes(String[] xs) {
            assert (xs.length % 3 == 0);
            if (xs.length == 0) {
                return Collections.emptyList();
            }
            List<Visitor.Attribute> attributes = new ArrayList<>();
            for (int i = 0; i < xs.length; i += 3) {
                attributes.add(new Visitor.Attribute(xs[i], xs[i + 1], xs[i + 2]));
            }
            return attributes;
        }

    }

}
