package com.github.foobar27.html5ever4j;

import java.util.List;
import java.util.Map;
import java.util.Optional;

public interface Sink<N> {

    N setDocType(String name, String _public, String system);
    N createText(String text);
    N createComment(String text);
    N createElement(String ns, String tagName, List<Map.Entry<String, String>> attributes);

}
