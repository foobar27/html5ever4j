package com.github.foobar27.html5ever4j.example;

import java.util.List;
import java.util.Map;

public class Node {

    private final String tag;
    private final Map<String, List<String>> attributes;
    private final List<Node> children;

    public Node(String tag, Map<String, List<String>> attributes, List<Node> children) {
        this.tag = tag;
        this.attributes = attributes;
        this.children = children;
    }
}
