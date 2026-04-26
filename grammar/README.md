# Grammar
Massive inspiration for the initial Grammar specification was taken from how Zig
[specifies their grammar](https://ziglang.org/documentation/master/#Grammar).
It is essentially Backus-Naur Form with some regex patterns mixed in. Production
rules work how they normally do in BNF, but if you see something with a '?',
like:

```
--------------------------------------------------------------------v
ExternalFunction := SKIP_WHITE KEYWORD_EXTERN FunctionDecl RingLevel?
```

This means this part of production may or may not be present (making that
syntactic element optional).

If you see something with a '*', like:

```
-------------------------v
Root             := Ingot* eof
```

This means that this part of the production may or may not be present (like '?')
but it may also be present multiple times.

SKIP_WHITE is used very generously in the GRAMMAR (and may need to be cleaned
up), but it represents exactly what it sounds like it does: a contiguous blob
of arbitrary whitespace characters of any length (yes, even the newline \n
character). This includes \n, \r, any any other ASCII characters that are
deemed as whitespace. Unicode characters that are deemed as whitespace but are
not ASCII will cause a Lexer error in this implementation.
