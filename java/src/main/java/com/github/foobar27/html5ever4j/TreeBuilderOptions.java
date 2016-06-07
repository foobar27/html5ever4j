package com.github.foobar27.html5ever4j;

import java.util.Objects;

public final class TreeBuilderOptions {

    private final boolean reportExactErrors;
    private final boolean scriptingEnabled;
    private final boolean iframeSrcdoc;
    private final boolean dropDoctype;
    private final QuirksMode quirksMode;

    private TreeBuilderOptions(Builder builder) {
        this.reportExactErrors = builder.reportExactErrors;
        this.scriptingEnabled = builder.scriptingEnabled;
        this.iframeSrcdoc = builder.iframeSrcdoc;
        this.dropDoctype = builder.dropDoctype;
        this.quirksMode = builder.quirksMode;
    }

    private NativeStruct nativeStruct;

    synchronized NativeStruct getNativeStruct() {
        if (nativeStruct == null) {
            nativeStruct = createNativeStruct();
        }
        return nativeStruct;
    }

    private NativeStruct createNativeStruct() {
        long ptr = Native.getInstance().createTreeBuilderOptions(this);
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
            return Native.treeBuilderOptionsToString(pointer);
        }

        @Override
        protected void finalize() throws Throwable {
            Native.destroyTreeBuilderOptions(pointer);
            super.finalize();
        }
    }


    @Override
    public String toString() {
        return String.format("TreeBuilderOptions[reportExactErrors=%b,scriptingEnabled=%b,iframeSrcdoc=%b,dropDoctype=%b,quirksMode=%s]",
                reportExactErrors,
                scriptingEnabled,
                iframeSrcdoc,
                dropDoctype,
                quirksMode);
    }

    @Override
    public int hashCode() {
        return Objects.hash(reportExactErrors, scriptingEnabled, iframeSrcdoc, dropDoctype, quirksMode);
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof TreeBuilderOptions))
            return false;
        TreeBuilderOptions that = (TreeBuilderOptions) t;
        return this.reportExactErrors == that.reportExactErrors
                && this.scriptingEnabled == that.scriptingEnabled
                && this.iframeSrcdoc == that.iframeSrcdoc
                && this.dropDoctype == that.dropDoctype
                && this.quirksMode == that.quirksMode;
    }

    /// {@link Builder#exactErrors(boolean)}
    public boolean reportExactErrors() {
        return reportExactErrors;
    }

    /// {@link Builder#scriptingEnabled(boolean)}
    public boolean scriptingEnabled() {
        return scriptingEnabled;
    }

    /// {@link Builder#iframeSrcdoc(boolean)}
    public boolean iframeSrcdoc() {
        return iframeSrcdoc;
    }

    /// {@link Builder#dropDoctype(boolean)}
    public boolean dropDoctype() {
        return dropDoctype;
    }

    /// {@link Builder#quirksMode(QuirksMode)}
    public QuirksMode quirksMode() {
        return quirksMode;
    }

    public Builder toBuilder() {
        return new Builder(this);
    }

    public static Builder newBuilder() {
        return new Builder();
    }

    public static final class Builder {
        private boolean reportExactErrors;
        private boolean scriptingEnabled;
        private boolean iframeSrcdoc;
        private boolean dropDoctype;
        private QuirksMode quirksMode;

        private Builder() {
            this.reportExactErrors = false;
            this.scriptingEnabled = false; // TODO verify
            this.iframeSrcdoc = false; // TODO verify
            this.dropDoctype = false; // TODO verify
            this.quirksMode = QuirksMode.NO_QUIRKS; // TODO verify
        }

        private Builder(TreeBuilderOptions options) {
            this.reportExactErrors = options.reportExactErrors;
            this.scriptingEnabled = options.scriptingEnabled;
            this.iframeSrcdoc = options.iframeSrcdoc;
            this.dropDoctype = options.dropDoctype;
            this.quirksMode = options.quirksMode;
        }

        @Override
        public String toString() {
            return String.format("TreeBuilderOptions.Builder[exactErrors=%b,scriptingEnabled=%b,iframeSrcdoc=%b,dropDoctype=%b,quirksMode=%s]",
                    reportExactErrors,
                    scriptingEnabled,
                    iframeSrcdoc,
                    dropDoctype,
                    quirksMode);
        }

        @Override
        public int hashCode() {
            return Objects.hash(reportExactErrors, scriptingEnabled, iframeSrcdoc, dropDoctype, quirksMode);
        }

        @Override
        public boolean equals(Object t) {
            if (!(t instanceof Builder))
                return false;
            Builder that = (Builder) t;
            return this.reportExactErrors == that.reportExactErrors
                    && this.scriptingEnabled == that.scriptingEnabled
                    && this.iframeSrcdoc == that.iframeSrcdoc
                    && this.dropDoctype == that.dropDoctype
                    && this.quirksMode == that.quirksMode;
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
         * Is scripting enabled?
         */
        public Builder scriptingEnabled(boolean b) {
            this.scriptingEnabled = b;
            return this;
        }

        /**
         * {@link Builder#scriptingEnabled(boolean)}
         */
        public boolean scriptingEnabled() {
            return scriptingEnabled;
        }

        /**
         * Is this an `iframe srcdoc` document?
         */
        public Builder iframeSrcdoc(boolean b) {
            this.iframeSrcdoc = b;
            return this;
        }

        /**
         * {@link Builder#iframeSrcdoc}
         *
         * @return
         */
        public boolean iframeSrcdoc() {
            return iframeSrcdoc;
        }

        /**
         * Should we drop the DOCTYPE (if any) from the tree?
         */
        public Builder dropDoctype(boolean b) {
            this.dropDoctype = b;
            return this;
        }

        /**
         * {@link Builder#dropDoctype(boolean)}
         */
        public boolean dropDoctype() {
            return dropDoctype;
        }

        /**
         * Initial TreeBuilder quirks mode.
         * Default: NoQuirks
         */
        public Builder quirksMode(QuirksMode m) {
            if (m == null) {
                throw new IllegalArgumentException("QuirksMode must not be null!");
            }
            this.quirksMode = m;
            return this;
        }

        /**
         * {@link Builder#quirksMode(QuirksMode)}
         */
        public QuirksMode quirksMode() {
            return quirksMode;
        }

        public TreeBuilderOptions build() {
            return new TreeBuilderOptions(this);
        }

    }

}
