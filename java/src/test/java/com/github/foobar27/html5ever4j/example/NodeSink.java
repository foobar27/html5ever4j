package com.github.foobar27.html5ever4j.example;

import com.github.foobar27.html5ever4j.Sink;
import com.github.foobar27.html5ever4j.Visitor;
import com.github.foobar27.html5ever4j.atoms.LocalName;
import com.github.foobar27.html5ever4j.atoms.Namespace;
import com.github.foobar27.html5ever4j.atoms.QualName;

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
    public Node createNormalElement(QualName name, List<Visitor.Attribute> attributes, List<Node> children) {
        return new Element(
                name.getNamespace().toString(),
                name.getLocalName().toString(),
                convertAttributes(attributes),
                children);
    }

    @Override
    public Node createScriptElement(QualName name, List<Visitor.Attribute> attributes, boolean alreadyStarted, List<Node> children) {
        return new ScriptElement(
                name.getNamespace().toString(),
                name.getLocalName().toString(),
                convertAttributes(attributes), alreadyStarted,
                children);
    }

    @Override
    public Node createTemplateElement(QualName name, List<Visitor.Attribute> attributes, List<Node> children) {
        return new TemplateElement(
                name.getNamespace().toString(),
                name.getLocalName().toString(),
                convertAttributes(attributes),
                children);
    }

    @Override
    public Node createAnnotationXmlElement(QualName name, List<Visitor.Attribute> attributes, boolean flag, List<Node> children) {
        return new AnnotationXmlElement(
                name.getNamespace().toString(),
                name.getLocalName().toString(),
                convertAttributes(attributes),
                flag,
                children);
    }

    private static Map<String, List<String>> convertAttributes(List<Visitor.Attribute> input) {
        // TODO should not ignore ns
        Map<String, List<String>> output = new HashMap<>();
        for (Visitor.Attribute a : input) {
            List<String> values = output.get(a.getKey().getLocalName().toString());
            if (values == null) {
                values = new ArrayList<>();
                output.put(a.getKey().getLocalName().toString(), values);
            }
            values.add(a.getValue());
        }
        return output;
    }

}
