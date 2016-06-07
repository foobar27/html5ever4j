package com.github.foobar27.html5ever4j;

import java.util.Objects;

public final class TokenizerOptions {

    private final boolean reportExactErrors;
    private final boolean discardBom;

    private TokenizerOptions(Builder builder) {
        this.reportExactErrors = builder.reportExactErrors;
        this.discardBom = builder.discardBom;
    }

    private NativeStruct nativeStruct;

    synchronized NativeStruct getNativeStruct() {
        if (nativeStruct == null) {
            nativeStruct = createNativeStruct();
        }
        return nativeStruct;
    }

    private TokenizerOptions.NativeStruct createNativeStruct() {
        long ptr = Native.getInstance().createTokenizerOptions(this);
        if (ptr == 0) {
            throw new OutOfMemoryError("Could not allocate native struct!");
        }
        return new TokenizerOptions.NativeStruct(ptr);
    }

    static class NativeStruct {
        final long pointer;

        NativeStruct(long pointer) {
            this.pointer = pointer;
        }

        @Override
        public String toString() {
            return Native.tokenizerOptionsToString(pointer);
        }

        @Override
        protected void finalize() throws Throwable {
            Native.destroyTokenizerOptions(pointer);
            super.finalize();
        }
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof TokenizerOptions))
            return false;
        TokenizerOptions that = (TokenizerOptions) t;
        return this.reportExactErrors == that.reportExactErrors
                && this.discardBom == that.discardBom;
    }

    @Override
    public int hashCode() {
        return Objects.hash(reportExactErrors, discardBom);
    }

    @Override
    public String toString() {
        return String.format("TokenizerOptions[reportExactErrors=%b,discardBom=%b]",
                reportExactErrors,
                discardBom);
    }

    /**
     * {@link Builder#reportExactErrors(boolean)}
     */
    public boolean reportExactErrors() {
        return reportExactErrors;
    }

    /**
     * {@link Builder#discardBom(boolean)}
     */
    public boolean discardBom() {
        return discardBom;
    }

    public Builder toBuilder() {
        return new Builder(this);
    }

    public static Builder newBuilder() {
        return new Builder();
    }

    public static final class Builder {
        private boolean reportExactErrors;
        private boolean discardBom;

        private Builder() {
            this.reportExactErrors = false;
            this.discardBom = true;
        }

        private Builder(TokenizerOptions options) {
            this.reportExactErrors = options.reportExactErrors;
            this.discardBom = options.discardBom;
        }

        @Override
        public boolean equals(Object t) {
            if (!(t instanceof TokenizerOptions))
                return false;
            Builder that = (Builder) t;
            return this.reportExactErrors == that.reportExactErrors
                    && this.discardBom == that.discardBom;
        }

        @Override
        public int hashCode() {
            return Objects.hash(reportExactErrors, discardBom);
        }

        @Override
        public String toString() {
            return String.format("TokenizerOptions.Builder[reportExactErrors=%b,discardBom=%b]",
                    reportExactErrors,
                    discardBom);
        }

        /**
         * Report all parse errors described in the spec, at some
         * performance penalty?
         * Default: false
         */
        public Builder reportExactErrors(boolean b) {
            this.reportExactErrors = b;
            return this;
        }

        /**
         * {@link Builder#reportExactErrors(boolean)}
         */
        public boolean reportExactErrors() {
            return reportExactErrors;
        }

        /**
         * Discard a `U+FEFF BYTE ORDER MARK` if we see one at the beginning
         * of the stream?
         * Default: true
         */
        public Builder discardBom(boolean b) {
            this.discardBom = b;
            return this;
        }

        /**
         * {@link Builder#discardBom(boolean)}
         */
        public boolean discardBom() {
            return discardBom;
        }

        public TokenizerOptions build() {
            return new TokenizerOptions(this);
        }

    }

}
