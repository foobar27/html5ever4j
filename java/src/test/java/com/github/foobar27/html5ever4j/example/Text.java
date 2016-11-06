package com.github.foobar27.html5ever4j.example;

import java.util.Collections;

public class Text extends Node {

    private final String text;

    public Text(String text) {
        super(Collections.emptyList());
        this.text = text;
    }

    @Override
    public String toString() {
        return String.format("Text[%s]", text);
    }

}
