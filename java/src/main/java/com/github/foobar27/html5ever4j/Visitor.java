package com.github.foobar27.html5ever4j;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public interface Visitor {

    void preOrderVisit();

    void setDocType(String name, String _public, String system);

    void createText(String text);
    void createComment(String text);
    void createNormalElement(String ns, String tagName, List<Attribute> attributes);
    void createScriptElement(String ns, String tagName, List<Attribute> attributes, boolean alreadyStarted);
    void createTemplateElement(String ns, String tagName, List<Attribute> attributes);
    void createAnnotationXmlElement(String ns, String tagName, List<Attribute> attributes, boolean flag); // TODO rename 'flag'

    final class Attribute {
        private final String namespace;
        private final String key;
        private final String value;

        public Attribute(String namespace, String key, String value) {
            this.namespace = namespace;
            this.key = key;
            this.value = value;
        }

        @Override
        public String toString() {
            return String.format("Attribute[ns=%s,key=%s,value=%s]", namespace, key, value);
        }

        public String getNamespace() {
            return namespace;
        }

        public String getKey() {
            return key;
        }

        public String getValue() {
            return value;
        }

    }


}
