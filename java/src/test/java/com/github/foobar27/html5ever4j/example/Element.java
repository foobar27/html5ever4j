package com.github.foobar27.html5ever4j.example;

import java.util.List;
import java.util.Map;

public class Element extends Node {

    private final String ns;
    private final String tag;
    private final Map<String, List<String>> attributes;

    public Element(String ns, String tag, Map<String, List<String>> attributes, List<Node> children) {
        super(children);
        this.ns = ns;
        this.tag = tag;
        this.attributes = attributes;
    }

    public String getNs() {
        return ns;
    }

    public String getTag() {
        return tag;
    }

    public Map<String, List<String>> getAttributes() {
        return attributes;
    }

    @Override
    public String toString() {
        return String.format("Element[%s,%s,%s,%s]",
                ns,
                tag,
                attributes,
                getChildren());
    }

}
