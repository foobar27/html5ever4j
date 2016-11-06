package com.github.foobar27.html5ever4j.example;

import java.util.List;
import java.util.Map;

public class AnnotationXmlElement extends Element {

    private final boolean flag;

    public AnnotationXmlElement(String ns, String tag, Map<String, List<String>> attributes, boolean flag, List<Node> children) {
        super(ns, tag, attributes, children);
        this.flag = flag;
    }

    public boolean getFlag() {
        return flag;
    }

    @Override
    public String toString() {
        return String.format("AnnotationXmlElement[%s,%s,%s,%b,%s]",
                getNs(),
                getTag(),
                getAttributes(),
                flag,
                getChildren());
    }

}
