package com.github.foobar27.html5ever4j.atoms;

public class Namespace extends Atom {

    private static final Atoms<Namespace> ATOMS = new Atoms<>(new String[]{
            "",
            "*",
            "http://www.w3.org/1999/xhtml",
            "http://www.w3.org/XML/1998/namespace",
            "http://www.w3.org/2000/xmlns/",
            "http://www.w3.org/1999/xlink",
            "http://www.w3.org/2000/svg",
            "http://www.w3.org/1998/Math/MathML"},
            Namespace::new,
            Namespace.class);

    private Namespace(int id, String string) {
        super(id, string);
    }

    public static Namespace getNamespace(int id, String string) {
        return ATOMS.getAtom(id, string);
    }

    @Override
    protected Atoms atoms() {
        return ATOMS;
    }

}
