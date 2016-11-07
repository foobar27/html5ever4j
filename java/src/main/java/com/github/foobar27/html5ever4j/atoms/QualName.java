package com.github.foobar27.html5ever4j.atoms;

import java.util.Objects;

public final class QualName {

    private final Namespace namespace;
    private final LocalName localName;

    public QualName(Namespace namespace, LocalName localName) {
        this.namespace = namespace;
        this.localName = localName;
    }

    public Namespace getNamespace() {
        return namespace;
    }

    public LocalName getLocalName() {
        return localName;
    }

    @Override
    public String toString() {
        return String.format("QualName[%s,%s]",
                namespace,
                localName);
    }

    @Override
    public int hashCode() {
        return Objects.hash(namespace, localName);
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof QualName)) {
            return false;
        }
        QualName that = (QualName) t;
        return this.namespace.equals(that.namespace)
                && this.localName.equals(that.localName);
    }

}
