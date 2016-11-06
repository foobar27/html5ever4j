package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.example.*;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.core.IsEqual.equalTo;
import static org.junit.Assert.assertEquals;
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
        Node root = Html5ever.parse("<p id=\"1\" class=\"bold\">foo", ParseOptions.newBuilder().build(), NodeSink::new);
        assertEquals("Element[http://www.w3.org/1999/xhtml,html,{},[Element[http://www.w3.org/1999/xhtml,head,{},[]], Element[http://www.w3.org/1999/xhtml,body,{},[Element[http://www.w3.org/1999/xhtml,p,{id=[1], class=[bold]},[Text[foo]]]]]]]",
                root.toString());
    }

    @Test
    public void parseShouldNoMixUpNeighbours() {
        Node root = Html5ever.parse("<ul><li>a1<li><li>b2<li></ul><ul><li>a1<li><li>b2<li></ul>", ParseOptions.newBuilder().build(), NodeSink::new);
        System.out.println(root);
        assertEquals("Element[http://www.w3.org/1999/xhtml,html,{},[Element[http://www.w3.org/1999/xhtml,head,{},[]], Element[http://www.w3.org/1999/xhtml,body,{},[Element[http://www.w3.org/1999/xhtml,ul,{},[Element[http://www.w3.org/1999/xhtml,li,{},[Text[a1]]], Element[http://www.w3.org/1999/xhtml,li,{},[]], Element[http://www.w3.org/1999/xhtml,li,{},[Text[b2]]], Element[http://www.w3.org/1999/xhtml,li,{},[]]]], Element[http://www.w3.org/1999/xhtml,ul,{},[Element[http://www.w3.org/1999/xhtml,li,{},[Text[a1]]], Element[http://www.w3.org/1999/xhtml,li,{},[]], Element[http://www.w3.org/1999/xhtml,li,{},[Text[b2]]], Element[http://www.w3.org/1999/xhtml,li,{},[]]]]]]]]",
                root.toString());
    }

}
