package com.github.foobar27.html5ever4j;

import org.junit.Test;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.core.IsEqual.equalTo;
import static org.junit.Assert.assertThat;

public class Html5everTest {

    @Test
    public void html2htmlShouldAddClosingTag() {
        String tidied = Html5ever.html2html(
                "<p>foo",
                ParseOptions.newBuilder().build(),
                SerializeOptions.newBuilder().build());
        assertThat(tidied, is(equalTo("<!DOCTYPE html>\n<html><head></head><body><p>foo</p></body></html>")));
    }

    @Test
    public void parseShouldAddClosingTag() {
        Parser.parse("<p>foo", ParseOptions.newBuilder().build());
        // TODO test something!
    }
}
