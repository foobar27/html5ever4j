package com.github.foobar27.html5ever4j;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class OptionsTest {

    @Test
    public void tokenizerOptionsNativeString() {
        // TODO generative testing!
        assertEquals(
                "TokenizerOpts[exact_errors=true,discard_bom=true]",
                nativeString(TokenizerOptions.newBuilder()
                        .discardBom(true)
                        .reportExactErrors(true)
                        .build()));
    }

    @Test
    public void treeBuilderOptionsNativeString() {
        // TODO generative testing!
        assertEquals(
                "TreeBuilderOpts[exact_errors=true,scripting_enabled=true,iframe_srcdoc=true,drop_doctype=true,quirks_mode=LimitedQuirks]",
                nativeString(TreeBuilderOptions.newBuilder()
                        .reportExactErrors(true)
                        .scriptingEnabled(true)
                        .iframeSrcdoc(true)
                        .dropDoctype(true)
                        .quirksMode(QuirksMode.LIMITED_QUIRKS)
                        .build()));
    }

    @Test
    public void parseOptionsNativeString() {
        // TODO generative testing!
        ParseOptions.Builder builder = ParseOptions.newBuilder();
        builder.tokenizerOptionsBuilder()
                .discardBom(true)
                .reportExactErrors(true);
        builder.treeBuilderOptionsBuilder()
                .reportExactErrors(true)
                .scriptingEnabled(true)
                .iframeSrcdoc(true)
                .dropDoctype(true)
                .quirksMode(QuirksMode.NO_QUIRKS);

        assertEquals(
                "ParseOpts[tokenizer=TokenizerOpts[exact_errors=true,discard_bom=true],tree_builder=TreeBuilderOpts[exact_errors=true,scripting_enabled=true,iframe_srcdoc=true,drop_doctype=true,quirks_mode=NoQuirks]]",
                nativeString(builder.build()));
    }

    private String nativeString(TokenizerOptions options) {
        return options.getNativeStruct().toString();
    }

    private String nativeString(TreeBuilderOptions options) {
        return options.getNativeStruct().toString();
    }

    private String nativeString(ParseOptions options) {
        return options.getNativeStruct().toString();
    }

}
