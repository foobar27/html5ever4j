package com.github.foobar27.html5ever4j.atoms;

import java.lang.reflect.Array;
import java.util.*;

public final class Atoms<A> {

    private final String[] id2name;
    private final A[] id2atom;
    private final Map<String, Integer> name2id;
    private final Constructor<A> ctor;

    Atoms(String[] names, Constructor<A> ctor, Class<A> c) {
        this.ctor = ctor;
        this.id2name = names;
        name2id = new HashMap<>();
        //noinspection unchecked
        id2atom = (A[]) Array.newInstance(c, names.length);
        for (int i = 0; i < names.length; ++i) {
            name2id.put(names[i], i);
            id2atom[i] = ctor.apply(i, names[i]);
        }
    }

    List<A> getAllAtoms() {
        return Collections.unmodifiableList(Arrays.asList(id2atom));
    }

    public A getAtom(int id, String s) {
        if (s == null) {
            return id2atom[id];
        } else {
            return ctor.apply(id, s);
        }
    }

    public String toString(int id, String s) {
        if (s == null) {
            return id2name[id];
        } else {
            return s;
        }
    }

    protected interface Constructor<A> {
        A apply(int id, String s);
    }

}