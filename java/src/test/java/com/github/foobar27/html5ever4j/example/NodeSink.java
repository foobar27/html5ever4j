package com.github.foobar27.html5ever4j.example;

import com.github.foobar27.html5ever4j.Sink;
import com.github.foobar27.html5ever4j.Visitor;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class NodeSink implements Sink<Node> {

    @Override
    public void setDocType(String name, String _public, String system) {
        // TODO don't ignore
    }

    @Override
    public Node createText(String text) {
        return new Text(text);
    }

    @Override
    public Node createComment(String text) {
        return new Comment(text);
    }

    @Override
    public Node createNormalElement(String ns, String tagName, List<Visitor.Attribute> attributes, List<Node> children) {
        return new Element(ns, tagName, convertAttributes(attributes), children);
    }

    @Override
    public Node createScriptElement(String ns, String tagName, List<Visitor.Attribute> attributes, boolean alreadyStarted, List<Node> children) {
        return new ScriptElement(ns, tagName, convertAttributes(attributes), alreadyStarted, children);
    }

    @Override
    public Node createTemplateElement(String ns, String tagName, List<Visitor.Attribute> attributes, List<Node> children) {
        return new TemplateElement(ns, tagName, convertAttributes(attributes), children);
    }

    @Override
    public Node createAnnotationXmlElement(String ns, String tagName, List<Visitor.Attribute> attributes, boolean flag, List<Node> children) {
        return new AnnotationXmlElement(ns, tagName, convertAttributes(attributes), flag, children);
    }

    private static Map<String, List<String>> convertAttributes(List<Visitor.Attribute> input) {
        // TODO should not ignore ns
        Map<String, List<String>> output = new HashMap<>();
        for (Visitor.Attribute a : input) {
            List<String> values = output.get(a.getKey());
            if (values == null) {
                values = new ArrayList<>();
                output.put(a.getKey(), values);
            }
            values.add(a.getValue());
        }
        return output;
    }

}
