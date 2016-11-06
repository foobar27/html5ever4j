package com.github.foobar27.html5ever4j;

import java.util.List;

public interface Sink<N> {

    void setDocType(String name, String _public, String system);

    N createText(String text);

    N createComment(String text);

    N createNormalElement(String ns, String tagName, List<Visitor.Attribute> attributes, List<N> children);

    N createScriptElement(String ns, String tagName, List<Visitor.Attribute> attributes, boolean alreadyStarted, List<N> children);

    N createTemplateElement(String ns, String tagName, List<Visitor.Attribute> attributes, List<N> children);

    N createAnnotationXmlElement(String ns, String tagName, List<Visitor.Attribute> attributes, boolean flag, List<N> children);
}
