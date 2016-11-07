package com.github.foobar27.html5ever4j.atoms;

import java.util.Objects;

public abstract class Atom {

    private final int id;
    private final String string;

    protected Atom(int id, String string) {
        this.id = id;
        this.string = string;
    }

    protected abstract Atoms atoms();

    public int getId() {
        return id;
    }

    public String getString() {
        return string;
    }

    @Override
    public String toString() {
        return atoms().toString(getId(), getString());
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, string);
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof Atom)) {
            return false;
        }
        Atom that = (Atom) t;
        if (this.string == null) {
            return that.string == null
                    && this.id == that.id;
        } else {
            return this.string.equals(that.string);
        }
    }

}
