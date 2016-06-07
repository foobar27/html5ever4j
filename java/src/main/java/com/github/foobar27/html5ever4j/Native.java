package com.github.foobar27.html5ever4j;

class Native {

    private static final String JNI_LIBRARY_NAME = "html5ever4j";

    private static final Native INSTANCE = new Native();

    private final long contextPointer;

    private Native() {
        // guaranteed to be called once (Singleton pattern)
        System.loadLibrary(JNI_LIBRARY_NAME);
        contextPointer = createContext();
        if (contextPointer == 0) {
            throw new IllegalArgumentException("Could not create contextPointer!");
        }
    }

    @Override
    protected void finalize() throws Throwable {
        destroyContext(contextPointer);
        super.finalize();
    }

    static Native getInstance() {
        return INSTANCE;
    }

//    public static native void parse(String input, Sink output, ParseOptions options);
//
//    public static void parseFragment() {
//
//    }
//    public static String html2text(String inputHtml) {
//
//    }

    private static native long createContext();
    private static native void destroyContext(long pointer);

    private static native long createTokenizerOptions(long wrapper, TokenizerOptions options);
    static native void destroyTokenizerOptions(long pointer);
    static native String tokenizerOptionsToString(long pointer);

    private static native long createTreeBuilderOptions(long wrapper, TreeBuilderOptions options);
    static native void destroyTreeBuilderOptions(long pointer);
    static native String treeBuilderOptionsToString(long pointer);

    private static native long createParseOptions(long wrapper, ParseOptions options);
    static native void destroyParseOptions(long pointer);

    private static native long createSerializeOptions(long wrapper,SerializeOptions options);
    static native void destroySerializeOptions(long pointer);

    static native String html2html(String inputHtml,
                                   long parseOptions,
                                   long serializeOptions);

    long createTokenizerOptions(TokenizerOptions options) {
        return createTokenizerOptions(contextPointer, options);
    }

    long createTreeBuilderOptions(TreeBuilderOptions options) {
        return createTreeBuilderOptions(contextPointer, options);
    }

    long createParseOptions(ParseOptions options) {
        return createParseOptions(contextPointer, options);
    }

    long createSerializeOptions(SerializeOptions options) {
        return createSerializeOptions(contextPointer, options);
    }

}
