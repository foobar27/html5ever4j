package com.github.foobar27.html5ever4j;

import java.util.Objects;

public final class SerializeOptions {

    private final boolean scriptingEnabled;
    private final TraversalScope traversalScope;

    private SerializeOptions(Builder builder) {
        this.scriptingEnabled = builder.scriptingEnabled;
        this.traversalScope = builder.traversalScope;
    }

    @Override
    public String toString() {
        return String.format("SerializeOptions[scriptingEnabled=%b,traversalScope=%s]",
                scriptingEnabled,
                traversalScope);
    }

    @Override
    public boolean equals(Object t) {
        if (!(t instanceof SerializeOptions))
            return false;
        SerializeOptions that = (SerializeOptions) t;
        return this.scriptingEnabled == that.scriptingEnabled
                && this.traversalScope == that.traversalScope;
    }

    @Override
    public int hashCode() {
        return Objects.hash(scriptingEnabled, traversalScope);
    }

    /// {@link Builder#scriptingEnabled(boolean)}
    public boolean scriptingEnabled() {
        return scriptingEnabled;
    }

    /// {@link Builder#traversalScope(TraversalScope)}
    public TraversalScope traversalScope() {
        return traversalScope;
    }

    public Builder toBuilder() {
        return new Builder(this);
    }

    public static Builder newBuilder() {
        return new Builder();
    }

    private NativeStruct nativeStruct;

    synchronized NativeStruct getNativeStruct() {
        if (nativeStruct == null) {
            nativeStruct = createNativeStruct();
        }
        return nativeStruct;
    }

    private NativeStruct createNativeStruct() {
        long ptr = Native.getInstance().createSerializeOptions(this);
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
        protected void finalize() throws Throwable {
            Native.destroySerializeOptions(pointer);
            super.finalize();
        }
    }


    public static final class Builder {

        private boolean scriptingEnabled;
        private TraversalScope traversalScope;

        private Builder() {
            this.scriptingEnabled = false; // TODO verify
            this.traversalScope = TraversalScope.CHILDREN_ONLY;
        }

        private Builder(SerializeOptions options) {
            this.scriptingEnabled = options.scriptingEnabled;
            this.traversalScope = options.traversalScope;
        }

        @Override
        public String toString() {
            return String.format("SerializeOptions[scriptingEnabled=%b,traversalScope=%s]",
                    scriptingEnabled,
                    traversalScope);
        }

        @Override
        public boolean equals(Object t) {
            if (!(t instanceof Builder))
                return false;
            Builder that = (Builder) t;
            return this.scriptingEnabled == that.scriptingEnabled
                    && this.traversalScope == that.traversalScope;
        }

        @Override
        public int hashCode() {
            return Objects.hash(scriptingEnabled, traversalScope);
        }

        /**
         * Is scripting enabled?
         */
        public Builder scriptingEnabled(boolean b) {
            this.scriptingEnabled = b;
            return this;
        }

        /// {@link Builder#scriptingEnabled(boolean)}
        public boolean scriptingEnabled() {
            return scriptingEnabled;
        }

        /**
         * Serialize the root node?
         * Default: ChildrenOnly
         */
        public Builder traversalScope(TraversalScope s) {
            if (s == null)
                throw new NullPointerException();
            this.traversalScope = s;
            return this;
        }

        /// {@link Builder#traversalScope(TraversalScope)}
        public TraversalScope traversalScope() {
            return traversalScope;
        }

        public SerializeOptions build() {
            return new SerializeOptions(this);
        }

    }

    enum TraversalScope {
        INCLUDE_NODE,
        CHILDREN_ONLY
    }

}
