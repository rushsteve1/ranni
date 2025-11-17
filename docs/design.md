# Design

> [!WARNING]
> This document will churn significantly during development.
> Check back frequently, assume nothing, ask questions!

Ranni is a meta-reductive interpreted programming language designed to merge
traditional functional/imperative programming, type-level declarations, and
macros into one concept.

It achieves this by evaluating the *functionally pure* parts of the source code,
including language features like imports and type checking, into a subset of the
language.
It is this step, which we call "compiling" though it acts more like JavaScript
Bundling, that allows us to combine type declarations and macros into one
imperative language.

In Ranni you don't write just code, you write code that evaluates to other code.
Ranni is most similar to Zig, where types are first-class and reflection takes
the places of a LISPy `quote` system.

## Notes about this design doc

This doc takes an example-driven approach. Formal definitions will be added
as time goes on, but first we should get a feel for the language.

For the most part I will be Rust and Ruby syntax highlighting, which will
inevitably be wrong. Perhaps one day GitHub will support Ranni highlighting!

When giving examples I may follow expressions with `#=` to indicate the
expression's value. The `#` hash character begins a comment, so these result
examples have no syntactic relevance.
Later when we discuss types and other more complex constant expressions I'll
keep using `#=` to show how these expressions expand into concrete values.

## Arithmetic

All programming languages are just fancy calculators, so let's start there.

Ranni understands 64-bit integers and floats, and a special 128-bit ratio type
that's used for integer division.
```rb
1 + 1 #= 2
1.0 + 2.0 #= 3.0

#  ratio types are always written using division
1 / 2 #= 1/2
1 + (1/2) #= 3/2

# Arithemetic with floats always results in a float
1 - 2.0 #= -1.0
(1/2) + 2.0 #= 2.5
```
This also demonstrates the only form of type coercion in Ranni:
Integers can become Ratios, and both can become Floats.

The arithmetic operators are (in order of precedence)
- Exponentiation `^`
- Multiplication `*`, Division `/`, and Modulo `%`
- Addition `+`, and Subtraction `-`

These and `.` dot are the __ONLY__ infix operators.
There are no prefix or postfix operators.

Underscores can be used in the middle (but not beginning or end) of number
literals, great for readability.
```rs
100_000
1.000_000_001
```

## Assignment

You can assign a value to a name using `let`
```rs
let x = 2
let y = x + 3
y #= 5
```

`let` is special for a number of reasons:
- It is the __only__ way to bind an assign a value to a name in a scope.
	Records and Structs can have named fields, but that is part of the structure
	not the current scope.
- Shadowing is allowed
- And, most importantly, when you assign using `let` it is **immutable**.
	Ranni does have mutability, but it is an unrelated system we'll get to later.
	So remember: you use `let` the value cannot be changed, only shadowed.

In Ranni everything is an expression and `let` returns the value that was
just assigned.

You do not need to assign a value to the name when using `let`, this is also
valid.
```rs
let x
```
This results in the value of `x` being *undefined*. When the interpreter
encounters an undefined type or value it immediately errors.

## Basic Compounds

Ranni supports two basic compound types.
In Ranni the `,` comma character is considered whitespace, and so is optional
in compound types.

Arrays are simple sequential lists of the __same type__.
They are written using `[ ]` square brackets and work much like you'd expect
from other languages.
Elements of an array are accessed using the `.` dot operator and the index
(starting from zero).
```rs
[1 2 3 4 5].2 #= 3

let x = [6 7 8]
let i = 2
x.i #= 8

# NOT allowed
[1 1.0 2.0 1/2]
```

The other compound is Records, which can hold values of different types
and are written using `( )` parentheses.
Like arrays the elements can be accessed by index
```rs
(1 2 3 4 5).2 #=3
(1 1.0 2.0 1/2).0 #= 1
```

Records have two additional features that make them very useful.
Firstly a record with a single element is the same as just that element.
This is called "tuple identity" in other languages, and importantly allows
records to be used as normal parentheses for precedence.
```rs
(1) #= 1
(1) + 2 #= 3
# Goes to any depth
((((1)) + (((2))))) #= 3
```

The other even more useful feature is that the elements of a record can be named.
Named fields must come after all unnamed fields, and can be accessed either by
index or by name.
```rs
let x = (1 2 3, foo = 4, bar = 5)
x.foo #= 4
x.3 #= 4

# NOT allowed
(1 foo = 2, 3, 4)
```
When indexing a record using a variable with the same name as a field,
wrap the right-hand side of the `.` dot expression in parens.
```rs
let foo = 1
let x = (1 2, foo = 3.0)
x.foo #= 3.0
x.(foo) #= 2
```

## Blocks

A common syntax in Ranni is a Block, which is used both to define a new lexical
scope as well as to organize code.
There are two forms of blocks, both inspired by JavaScript.

`=>` "Arrow" blocks expect a single expression, they're a useful shorthand in
many places. The expression value of the block is the same as the expression.
```rs
=> 1 + 1 #= 2
=> foo.bar
```

"Body" blocks use `{ }` curly braces and can take multiple expressions.
The expression value of the whole block is the final expression.
```rs
{
	1 + 2
	foo.bar
	3 + 4
} #= 7
```

In any case where Ranni expects a block either syntax can be used.

## Control Flow

In Ranni there is only one core control flow operator: `match`.
We'll discuss it in much more depth later in Pattern Matching,
but for now I want to introduce it and build some intuition.

The Match form is inspired by Rust, but with a few tweaks
```rs
let x = 15
match x / 3 {
	case 5 => $ * 2
	case _ => $ * 3
} #= 10
```

The format of a match is...
- The keyword `match`
- The expression to be matched
- A block containing a series of `case` expressions, each of which attempts
	to match against their provided expression.
- Each case has then has an "arm" block

The result of a Match is the same as the matched arm.

This is also our first encounter with `$` the anonymous variable.
In places where an argument is implicit, such as the expression in a match,
it is assigned to the `$` dollar sign variable.

Alternatively, we could use the fact that `let` returns its value.
```rs
let x = 15
match let y = x / 3 {
	case 5 => y * 2
	case _ => y * 3
} #= 10
```

## Functions

Now we get into the meat that separates a calculator from a computer!

Functions are a first-class part of Ranni, acting just like other values.
You assign them with `let` as always.
Calling a function is done by following it with a record, which looks a lot
like other languages.

And so in classic tradition
```rs
let fib = fn (n) Int {
	match n {
		case 0 => 0
		case 1 => 1
		case _ => recur($ - 1) + recur($ - 2)
	}
}

fib(6) #= 8
```

The format of a function is...
- The keyword `fn`
- An optional Record expression
- An optional expression for the return type
- Finally the block, also optional (useful for function signatures)

Functions cannot know their own names (that's bound after the function is created)
so we use the `recur` special variable to refer to the current function.
This is especially nice since it means that we no longer need to bind a function
to a name at all to do recursion.

When the argument record is omitted you can use the anonymous variable
to access the arguments.
Multiple arguments can be accessed by index using `$1`, `$2`, etc. where
`$0` is the same as just `$` (this is true for match as well).
```rs
let square = fn => $ * $
let add = fn $0 + $1
```

## First-Class Types

In Ranni all types are first-class. They can be assigned to variables and passed
as arguments to and from functions. Any place that expects a type actually
expects an *expression that evaluates to a type*.

All type values are of the special meta-type named `Type`.
By convention types are named in Pascal case.

Since Ranni is expanding all values at compile time this allows us to construct
concrete implementations of generic functions using a technique called
*specialization*.
```rs
let square_factory = fn(T: Type) => fn (n: T) => n * n
```
This is a function that takes in a type `T` and returns another function that
takes in a value *of type `T`* and then multiplies it times itself.
`T` is used to specialize the returned function.

### Built-In Data Types

Number types
- `Byte` 8 bits
- `Int` 64 bits
- `Float` 64 bits
- `Ratio` 128 bits

Meta Types
- `Type` any type value
- `Array` any array, regardless of underlying type
- `Record` any record, regardless of shape
- `Struct` any struct, regardless of shape (preserves uniqueness)
- `Func` any function, regardless of signature

Compiler types
- `Module` a code module
- `Fiber` a running coroutine

### Predicate Types

## Structs

In Ranni Structs are the only form of user-defined type, combining the features
of Structs and Enums in other languages like Rust.

Structs are defined using the `struct` keyword followed by a block of either
a list types, field assignments, or `case` expressions.

```rs
# Simple "newtype" structs can have only one inner value
let Miles = struct => Float
```
```rs
# Field names are optional, allowing for tuple-like structs
let Pair = struct { Int, Int }
```
```rs
let Vector = struct {
	x: Int y: Int z: Int
}
```

Under the hood structs function are backed by records. So constructing an
instance of a struct is done much like calling a function, by passing a matching
underlying record.
```rs
Pair(4, 5)
Vector(x = 1.0, y = 0.0, z = 3.0)
```

Cases are defined with the `case` keyword, mirroring the same use in `match`.
```rs
let Distance = struct {
	case Miles = Float
	case FeetAndInches = struct { feet = Int, inches = Int }
}
```
Case expressions form new sub-types, and are constructed and matched directly.
```rs
let x = Distance.FeetAndInches(feet = 5, inches = 10)
match x {
	case Distance.FeetAndInches{ f, i } => i + (f * 12)
	case Distance.Miles(m) => m * 63_360
}
```

## Path Values

Paths are a special datatype that allows for referring to places within other
data types. They function similarly to symbols in LISP, but can encode even
more information.

"free" path values can be created with the `:.` syntax
```rs
# The first element of something
:.0
# A path to a filed named "foo"
:.foo
# A more complex chained path with a method call
:.0.foo.3.bar().baz
```

To interpolate a value into a path wrap it in parentheses
```rs
let x = 1
:.foo.(x).bar #= :.foo.1.bar
```

The `.` dot operator is used to index a value by a path, as we've seen with
arrays, records, and structs. We can also use paths from variables, again
interpolating using parens.
```rs
let my_path = :.foo.bar.baz
my_complex_type.(my_path)
```

## Pragmas

Pragmas, which look similar to `let` but use the `pragma` keyword,
are used to give information to the compiler about the current item.
These can be used in a wide variety of different ways, depending on the
compiler.
```rs
struct {
	pragma inheritable = true
}
```

Note that pragmas apply to the thing that they are inside.
At the top level of a file that means the current module.

## Pattern Matching

## Effects

## Mutability

## Concurrency

## Parallelism

## Error Handling

Ranni supports two varieties of error handling.

First is exceptions, which immediately halt the current fiber and signal the
parent. This is not intended to be handled by the programmer, and is reserved
for operations from which the interpreter may not be able to recover.
This is for divide-by-zero, referencing undefined variables, and other fatal
error conditions.

Second, and more commonly, is the monadic error handling found in languages like
Rust using `Result` and `Option`. These types are defined in the standard
library's `core` module that is imported by default.

## Modules

Modules are the unit of compilation for Ranni.
Each file is a module and can import other modules.

They `import` keyword takes a single argument, a URI to the module to import.
If a protocol prefix is not provided it is assumed to be `file:` and relative
to **the current module**. Not to any workspace root or the CWD.

Import returns the module as a value, so in most cases you'll want to assign
it or destructure out individual items.
```rs
# We're in the file ~/dev/blah.rni
# And we're importing the file ~/dev/other.rni
let other_module = import "other.rni"
other_module.foo()

# Alternatively we can destructure items out
let { foo } = import "other.rni"
```

Yes a URI, we're leaving the door open for Deno-style network imports.
Import also does not strictly import only Ranni files. Other filetypes could
be supported if they can be parsed to a Ranni structure (i.e. JSON).

## Virtual Effects and Macros

Some effects are "virtual", unlike regular effects the compiler **does** know
how to handle them and they will be called for the effect at compile time.
These effects are used to depend on the compilation environment, or even on the
code being compiled itself.

Using virtual effects we can perform complex meta-programming tasks as simply
as writing imperative code.
```rs
# The comp.UpScope virtual effect can be used to alter the calling scope
let my_macro = fn <comp.UpScope> {
	UpScope.let(:foo, 5)
	UpScope.let(:bar, 2)
}

let my_func = fn {
	my_macro()
	foo * bar #= 10
}
```

```rs
# comp.ModScope is like comp.UpScope but applies to the calling module
let derive_print = fn <comp.ModScope> (T: type) {
	ModScope.let_method(:print, fn {
		# This is not a very good printing function, but still a useful example
		let fields = T.cases.map(fn => $.fields.map(fn => "\n\t{$.name}: {$.type} = {$.value}"))
		print("{T.name} \{{fields...}\n\}""
	})
}

let MyStruct = struct {
	foo: Int,
	bar: Float
}

# Unlike LISP we don't use quotation, instead passing types as values
derive_print(MyStruct)

MyStruct(foo: 5, bar: 1.0).print()
# prints...
MyStruct {
	foo: Int = 5
	bar: Float = 1.0
}
```

These effects disappear during compilation since the compiler is able to expand
them fully and erase the effect.

Macros are a powerful feature that should be used with care.
By forcing macro functions to include a virtual effect we let callers know
that their code might be affected.

## Methods

## Conclusion

And that's it! That's all of the core Ranni programming language.
You might have noticed we sort of skipped quite a bit.
No mention of strings or characters, and goodness not even booleans?

All of these things (and more) are defined in the Ranni standard library.
