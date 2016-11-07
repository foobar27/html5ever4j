package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.atoms.LocalName;
import com.github.foobar27.html5ever4j.atoms.Namespace;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

class SinkVisitor<N> implements Visitor {

    private final Sink<N> sink;
    private final Stack<List<N>> childrenStack = new Stack<>();

    SinkVisitor(Sink<N> sink) {
        this.sink = sink;
        childrenStack.push(new ArrayList<>()); // will contain the root node
    }

    public N getParsedRoot() {
        assert (childrenStack.size() == 1);
        assert (childrenStack.peek().size() == 1);
        return childrenStack.peek().get(0);
    }

    @Override
    public void preOrderVisit() {
        childrenStack.push(new ArrayList<>());
    }

    @Override
    public void setDocType(String name, String _public, String system) {
        sink.setDocType(name, _public, system);
    }

    @Override
    public void createText(String text) {
        childrenStack.peek().add(sink.createText(text));
    }

    @Override
    public void createComment(String text) {
        childrenStack.peek().add(sink.createComment(text));
    }

    @Override
    public void createNormalElement(Namespace ns, LocalName tag, List<Attribute> attributes) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createNormalElement(ns, tag, attributes, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createScriptElement(Namespace ns, LocalName tag, List<Attribute> attributes, boolean alreadyStarted) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createScriptElement(ns, tag, attributes, alreadyStarted, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createTemplateElement(Namespace ns, LocalName tag, List<Attribute> attributes) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createTemplateElement(ns, tag, attributes, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createAnnotationXmlElement(Namespace ns, LocalName tag, List<Attribute> attributes, boolean flag) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createAnnotationXmlElement(ns, tag, attributes, flag, children);
        childrenStack.peek().add(newElement);
    }

}
