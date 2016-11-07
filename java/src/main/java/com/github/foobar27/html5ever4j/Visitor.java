package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.atoms.LocalName;
import com.github.foobar27.html5ever4j.atoms.Namespace;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public interface Visitor {

    void preOrderVisit();

    void setDocType(String name, String _public, String system);

    void createText(String text);
    void createComment(String text);
    void createNormalElement(Namespace ns, LocalName tagName, List<Attribute> attributes);
    void createScriptElement(Namespace ns, LocalName tagName, List<Attribute> attributes, boolean alreadyStarted);
    void createTemplateElement(Namespace ns, LocalName tagName, List<Attribute> attributes);
    void createAnnotationXmlElement(Namespace ns, LocalName tagName, List<Attribute> attributes, boolean flag); // TODO rename 'flag'

    final class Attribute {
        private final Namespace namespace;
        private final LocalName key;
        private final String value;

        public Attribute(Namespace namespace, LocalName key, String value) {
            this.namespace = namespace;
            this.key = key;
            this.value = value;
        }

        @Override
        public String toString() {
            return String.format("Attribute[ns=%s,key=%s,value=%s]", namespace, key, value);
        }

        public Namespace getNamespace() {
            return namespace;
        }

        public LocalName getKey() {
            return key;
        }

        public String getValue() {
            return value;
        }

    }


}
