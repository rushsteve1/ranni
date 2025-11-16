# Ranni

An interpreted meta-reductive programming language inspired by Zig and LISP

## The Big Idea

Most programming languages (C, Rust, JS, Swift, etc.) are really 2-3
languages in a trench coat, working together to pretend to be only one.
These are usually...
- The imperative language inside functions
- A declarative language for type logic or templates
- Something else for macros

Ranni intends to try and follow in the footsteps of languages like LISP and Zig
to provide only *one* language that does everything.
Our approach to this is through "meta-reduction".

Simply put you write code, and in that code you have to annotate the *effects*
that functions use. Effects can be things like networking or file I/O,
or they can be contextual values like the current line number or calling function.
Effects are all of the "impure" things that the code does.

By tracking these effects (or in the case of line numbers, erasing them)
we can know which parts of the program are pure and which are not.
Thus the "compile" step is instead evaluating all the pure parts of the program,
resulting in a new program that has been optimized into only the effectful parts.

Since we're capable of evaluating pure code ahead of time, we can use this to
implement the other parts of the language (types and macros) using
simple imperative code.

## The Other Idea

The second part of Ranni as a project is to ask a bit more of our tools.
The reference Ranni interpreter intends to be extremely transparent,
providing many tools to analyze and debug your code.

To that end the interpreter embeds a web-server as the interface for
various analysis. Support for LSP, DAP, and OpenTelemetry is planned!
