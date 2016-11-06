package com.github.foobar27.html5ever4j.example;

import java.util.List;
import java.util.Map;

public class TemplateElement extends Element {

    public TemplateElement(String ns, String tag, Map<String, List<String>> attributes, List<Node> children) {
        super(ns, tag, attributes, children);
    }

    @Override
    public String toString() {
        return String.format("TemplateElement[%s,%s,%s,%b,%s]",
                getNs(),
                getTag(),
                getAttributes(),
                getChildren());
    }

}
