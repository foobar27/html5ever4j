package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.example.Node;
import org.junit.Test;

import java.util.List;

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
        Html5ever.parse("<p id=\"1\" class=\"bold\">foo", ParseOptions.newBuilder().build(), new Sink<Node>() {
            @Override
            public Node setDocType(String name, String _public, String system) {
                System.out.println(String.format("setDocType(%s,%s,%s)", name, _public, system));
                return null; // TODO
            }

            @Override
            public Node createText(String text) {
                System.out.println(String.format("createText(%s)", text));
                return null; // TODO
            }

            @Override
            public Node createComment(String text) {
                System.out.println(String.format("createComment(%s)", text));
                return null; // TODO
            }

            @Override
            public Node createNormalElement(String ns, String tagName, List<Attribute> attributes) {
                System.out.println(String.format("createNormalElement(%s, %s, %s)", ns, tagName, attributes));
                return null; // TODO
            }

            @Override
            public Node createScriptElement(String ns, String tagName, List<Attribute> attributes, boolean alreadyStarted) {
                System.out.println(String.format("createScriptElement(%s, %s, %s, %b)", ns, tagName, attributes, alreadyStarted));
                return null; // TODO
            }

            @Override
            public Node createTemplateElement(String ns, String tagName, List<Attribute> attributes) {
                System.out.println(String.format("createTemplateElement(%s, %s, %s)", ns, tagName, attributes));
                return null; // TODO
            }

            @Override
            public Node createAnnotationXmlElement(String ns, String tagName, List<Attribute> attributes, boolean flag) {
                System.out.println(String.format("createAnnotationXmlElement(%s, %s, %s, %b)", ns, tagName, attributes, flag));
                return null; // TODO
            }

        });
        // TODO test something!
    }
}
