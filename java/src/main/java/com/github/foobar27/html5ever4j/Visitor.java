package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.atoms.QualName;

import java.util.List;

public interface Visitor {

    void preOrderVisit();

    void setDocType(String name, String _public, String system);

    void createText(String text);
    void createComment(String text);
    void createNormalElement(QualName name, List<Attribute> attributes);
    void createScriptElement(QualName name, List<Attribute> attributes, boolean alreadyStarted);
    void createTemplateElement(QualName name, List<Attribute> attributes);
    void createAnnotationXmlElement(QualName name, List<Attribute> attributes, boolean flag); // TODO rename 'flag'

    final class Attribute {
        private final QualName key;
        private final String value;

        public Attribute(QualName key, String value) {
            this.key = key;
            this.value = value;
        }

        @Override
        public String toString() {
            return String.format("Attribute[key=%s,value=%s]", key, value);
        }

        public QualName getKey() {
            return key;
        }

        public String getValue() {
            return value;
        }

    }


}
