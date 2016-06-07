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
                "TreeBuilderOpts[exact_errors=true,scripting_enabled=true,iframe_srcdoc=true,drop_doctype=true,quirks_mode=NoQuirks]",
                nativeString(TreeBuilderOptions.newBuilder()
                        .reportExactErrors(true)
                        .scriptingEnabled(true)
                        .iframeSrcdoc(true)
                        .dropDoctype(true)
                        .quirksMode(QuirksMode.NO_QUIRKS)
                        .build()));
    }

    private String nativeString(TokenizerOptions options) {
        return options.getNativeStruct().toString();
    }

    private String nativeString(TreeBuilderOptions options) {
        return options.getNativeStruct().toString();
    }
}
