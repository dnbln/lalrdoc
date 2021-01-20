# Contributing

First, you need to know: this codebase was taken from lalrpop's
own source code, and the "backend", e.g. the code that actually
generates the parser was removed.

Given lalrpop doesn't produce concrete syntax trees, the inner
and outer doc comments are parsed manually in the grammar
and stored inside the `NonterminalData` and `Alternative`
structures. Regular comments are still skipped by the lexer.
That means the only places where outer doc comments are allowed are:

```text
/// here, *always* in front of the attributes
#[inline]
Nonterminal = {
    /// and here, *always* in front of any attributes on alternatives
    #[precedence(level="1")]
    A "+" B,
    /// Also here
    A "-" B,
}
```

While inner doc comments are only allowed here:

```text
Nonterminal = {
    //! inner doc comment here, *always* before any of the alternatives.

    /// doc comment on alternative
    A "+" B
}
```

And the parser will reject doc comments in any other places than those specified above,
even if lalrpop would normally accept them.

Implementing a reference builder pretty much only means implementing `ReferenceBuilder`
(see the `reference_builder` module) for one of your own types and adding it in `cli.rs`.