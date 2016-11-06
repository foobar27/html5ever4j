package com.github.foobar27.html5ever4j.example;

import java.util.Collections;

public class Comment extends Node {

    private final String text;

    public Comment(String text) {
        super(Collections.emptyList());
        this.text = text;
    }

    @Override
    public String toString() {
        return String.format("Comment[%s]", text);
    }

}
