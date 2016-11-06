package com.github.foobar27.html5ever4j.example;

import java.util.List;
import java.util.Map;

public class ScriptElement extends Element {

    private final boolean alreadyStarted;

    public ScriptElement(String ns, String tag, Map<String, List<String>> attributes, boolean alreadyStarted, List<Node> children) {
        super(ns, tag, attributes, children);
        this.alreadyStarted = alreadyStarted;
    }

    public boolean isAlreadyStarted() {
        return alreadyStarted;
    }

    @Override
    public String toString() {
        return String.format("ScriptElement[%s,%s,%s,%b,%s]",
                getNs(),
                getTag(),
                getAttributes(),
                alreadyStarted,
                getChildren());
    }

}
