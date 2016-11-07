package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.atoms.QualName;

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
    public void createNormalElement(QualName name, List<Attribute> attributes) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createNormalElement(name, attributes, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createScriptElement(QualName name, List<Attribute> attributes, boolean alreadyStarted) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createScriptElement(name, attributes, alreadyStarted, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createTemplateElement(QualName name, List<Attribute> attributes) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createTemplateElement(name, attributes, children);
        childrenStack.peek().add(newElement);
    }

    @Override
    public void createAnnotationXmlElement(QualName name, List<Attribute> attributes, boolean flag) {
        List<N> children = childrenStack.pop();
        N newElement = sink.createAnnotationXmlElement(name, attributes, flag, children);
        childrenStack.peek().add(newElement);
    }

}
