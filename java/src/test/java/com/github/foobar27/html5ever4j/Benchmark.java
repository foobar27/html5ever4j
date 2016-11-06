package com.github.foobar27.html5ever4j;

import com.github.foobar27.html5ever4j.example.NodeSink;
import com.google.common.base.Charsets;
import com.google.common.io.Files;

import java.io.File;
import java.io.IOException;

public class Benchmark {

    private static void benchmark(Runnable runnable) {
        for (int i = 0; i < 10; ++i) {
            runnable.run();
        }
        long begin = System.currentTimeMillis();
        int nIter = 1000;
        for (int i = 0; i < nIter; ++i) {
            runnable.run();
        }
        long end = System.currentTimeMillis();
        System.out.println("took: " + (double)(end - begin) / nIter);
    }

    public static void main(String[] args) throws IOException {
        String html = Files.toString(new File(args[0]), Charsets.UTF_8);
        ParseOptions parseOptions = ParseOptions.newBuilder().build();
        SerializeOptions serializeOptions = SerializeOptions.newBuilder().build();
        System.out.println("Parsing benchmark...");
        benchmark(() -> Html5ever.parse(html, parseOptions, NodeSink::new));
        System.out.println("Html2html benchmark...");
        benchmark(() -> Html5ever.html2html(html, parseOptions, serializeOptions));
    }

}
