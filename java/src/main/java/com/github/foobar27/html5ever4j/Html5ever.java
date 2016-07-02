package com.github.foobar27.html5ever4j;

public final class Html5ever {

    public static String html2html(String inputHtml, ParseOptions parseOptions, SerializeOptions serializeOptions) {
        return Native.getInstance().html2html(
                inputHtml,
                parseOptions.getNativeStruct().pointer,
                serializeOptions.getNativeStruct().pointer);
    }

    public static <N> void parse(String inputHtml, ParseOptions parseOptions, Sink<N> sink) {
        new Parser<N>(parseOptions).parse(inputHtml, sink);
    }

}
