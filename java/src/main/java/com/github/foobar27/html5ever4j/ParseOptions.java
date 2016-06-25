package com.github.foobar27.html5ever4j;

import java.util.Objects;

public final class ParseOptions {

    private final TokenizerOptions tokenizerOptions;
    private final TreeBuilderOptions treeBuilderOptions;

    private ParseOptions(Builder builder) {
        this.tokenizerOptions = builder.tokenizerOptionsBuilder.build();
        this.treeBuilderOptions = builder.treeBuilderOptionsBuilder.build();
    }

    @Override
    public String toString() {
        return String.format("ParseOptions[tokenizerOptions=%s,treeBuilderOptions=%s]",
                tokenizerOptions,
                treeBuilderOptions);
    }

    @Override
    public int hashCode() {
        return Objects.hash(tokenizerOptions, treeBuilderOptions);
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof ParseOptions))
            return false;
        ParseOptions that = (ParseOptions) t;
        return this.tokenizerOptions.equals(that.tokenizerOptions)
                && this.treeBuilderOptions.equals(that.treeBuilderOptions);
    }

    public TokenizerOptions tokenizerOptions() {
        return tokenizerOptions;
    }

    public TreeBuilderOptions treeBuilderOptions() {
        return treeBuilderOptions;
    }

    private NativeStruct nativeStruct;

    synchronized NativeStruct getNativeStruct() {
        if (nativeStruct == null) {
            nativeStruct = createNativeStruct();
        }
        return nativeStruct;
    }

    private NativeStruct createNativeStruct() {
        long ptr = Native.getInstance().createParseOptions(this);
        if (ptr == 0) {
            throw new OutOfMemoryError("Could not allocate native struct!");
        }
        return new NativeStruct(ptr);
    }

    static class NativeStruct {
        final long pointer;

        NativeStruct(long pointer) {
            this.pointer = pointer;
        }

        @Override
        public String toString() {
            return Native.parseOptionsToString(pointer);
        }

        @Override
        protected void finalize() throws Throwable {
            Native.destroyParseOptions(pointer);
            super.finalize();
        }
    }

    public Builder toBuilder() {
        return new Builder(this);
    }

    public static Builder newBuilder() {
        return new Builder();
    }

    public static final class Builder {
        private final TokenizerOptions.Builder tokenizerOptionsBuilder;
        private final TreeBuilderOptions.Builder treeBuilderOptionsBuilder;

        private Builder() {
            this.tokenizerOptionsBuilder = TokenizerOptions.newBuilder();
            this.treeBuilderOptionsBuilder = TreeBuilderOptions.newBuilder();
        }

        private Builder(ParseOptions options) {
            this.tokenizerOptionsBuilder = options.tokenizerOptions.toBuilder();
            this.treeBuilderOptionsBuilder = options.treeBuilderOptions.toBuilder();
        }

        @Override
        public String toString() {
            return String.format("ParseOptions.Builder[tokenizerOptions=%s,treeBuilderOptions=%s]",
                    tokenizerOptionsBuilder,
                    treeBuilderOptionsBuilder);
        }

        @Override
        public int hashCode() {
            return Objects.hash(tokenizerOptionsBuilder, treeBuilderOptionsBuilder);
        }

        @Override
        public boolean equals(Object t) {
            if (!(t instanceof Builder))
                return false;
            Builder that = (Builder) t;
            return this.tokenizerOptionsBuilder.equals(that.tokenizerOptionsBuilder)
                    && this.treeBuilderOptionsBuilder.equals(that.treeBuilderOptionsBuilder);
        }


        public TokenizerOptions.Builder tokenizerOptionsBuilder() {
            return tokenizerOptionsBuilder;
        }

        public TreeBuilderOptions.Builder treeBuilderOptionsBuilder() {
            return treeBuilderOptionsBuilder;
        }

        public ParseOptions build() {
            return new ParseOptions(this);
        }

    }

}
