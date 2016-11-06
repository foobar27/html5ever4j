package com.github.foobar27.html5ever4j;

import java.util.function.Supplier;

public final class Html5ever {

    public static String html2html(String inputHtml, ParseOptions parseOptions, SerializeOptions serializeOptions) {
        return Native.getInstance().html2html(
                inputHtml,
                parseOptions.getNativeStruct().pointer,
                serializeOptions.getNativeStruct().pointer);
    }

    public static void parse(String inputHtml, ParseOptions parseOptions, Visitor visitor) {
        new Parser(parseOptions).parse(inputHtml, visitor);
    }

    public static <N> N parse(String inputHtml, ParseOptions parseOptions, Supplier<Sink<N>> sinkFactory) {
        SinkVisitor<N> sink = new SinkVisitor<>(sinkFactory.get());
        parse(inputHtml, parseOptions, sink);
        return sink.getParsedRoot();
    }

}
