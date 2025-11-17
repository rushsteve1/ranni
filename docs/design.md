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
the places of a LISP-y `quote` system.

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
	So remember: when you use `let` the value cannot be changed, only shadowed.

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
Both types are immutable, like all values in Ranni.
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

### Calling Functions

I mentioned that calling functions was "like normal" with parentheses.
But Ranni has a few tricks too: Currying and Overruns.

Currying happens when you call a function with fewer arguments than it expected.
More specifically when the passed record is a *sub-match*.
When this happens instead of calling the function immediately and giving its
return value, it will return a new function that takes the remaining arguments
```rs
let foo = fn (a, b, c) {
	a + b + c
}

let one_two = foo(1, 2) #= fn (c)
one_two(3) #= 6
```

The second feature is the opposite, and as far as I know unique to Ranni.
When a function returns another function the arguments to the two can be combined
into a single call.
```rs
# A fn that returns a fn
let foo = fn (a, b) => fn (c) {
	a + b + c
}

# In this case same as above
let one_two = foo(1, 2) #= fn (c)
one_two(3) #= 6

# Or we use the overrun
foo(1, 2, 3) #= 6
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

You can use both top level fields and cases, the top-level fields will be shared
between the cases.
```rs
let Blah = struct {
	shared: Int
	other: Float

	case Yada = struct {
		# Has the shared fields and this one
		bar: (Int, Int)
	}
}

# Needs all the fields
Blah.Yada(shared = 1, other = 1.2, bar = (3, 4))
```

### Inheritance

Ranni supports a limited form of single inheritance.

In Java/C++ terms the default for structs is a kind of `abstract final`.
Cases on a struct automatically form child types, and by default ONLY cases
within the struct are child types, and you cannot construct the outer struct.
Both of these features can be controlled with pragmas.

First lets look at non-abstract types
```rs
let NonAbstract = struct {
	pragma abstract = false

	shared: Int

	case Inner = struct {
		not_shared: Ratio
	}
}

# We can construct the outer type
let x = NonAbstract(shared: 1)
let y = NonAbstract.Inner(shared: 1, not_shared: 1/3)

# Matching must still be exhaustive
match y {
	case NonAbstract(s) => s/1
	case NonAbstract.Inner(s, ns) => s + ns
} #= 4/3
```

Non-final type can be externally inherited from using the `inherits` keyword in
a struct.
```rs
let NonFinal = struct {
	pragma final = false

	base: Int
}

# Not final but still abstract!
NonFinal(base: 5) # NOT allowed

let Child = struct {
	# inherits must be the first thing in a struct
	inherits NonFinal

	inner: Float

	# we can combine with cases as well, Child is back to abstract final
	case Other = Int
}

Child.Other(3, inner: 2.2)
```

And of course we can combine them to form the more traditional OOP-y style
```rs
let Quad = struct {
	pragma abstract = false
	pragma final = false

	top: Int
	bottom: Int
	left: Int
	right: Int
}

let Square = struct {
	inherits Quad

	# We can replace the constructor, which can access the default with Self
	ctor = fn (Int) Self => Self(top: $, bottom: $, left: $, right: $)
}

let Parallelo = struct {
	inherits Quad

	ctor = fn (top: Int, bottom: Int, side: Int) Self => Self(top: top, bottonm: bottom, left: side, right: side)
}

let q = Quad(top: 1, bottom: 1, left: 1, right: 1)
let s = Square(1)
let p = Parallelo(top: 1, bottom: 2, sides: 3)
```

Inheritance like this is not considered idiomatic for Ranni,
and is intentionally limited. But I still consider it extremely useful,
and is a simple generalization of the ADTs that structs already provide.
This is *similar* to GADTs, but not strictly the same.

## Generics

By combining functions, first-class types, and structs we can create generic
data structures!
```rs
# From the std lib
let Option = fn (T: Type) => struct {
	case Some = T
	case None = void
}
```

And of course generic functions too
```rs
let gen_foo = fn (T: Type) => fn (a: T, b: T) => a + b
gen_foo(Int)(1, 2) #= 3
# This is where overruns come in handy!
gen_foo(Int, 1, 2)
```

Generics benefit a lot from type inference, which we'll discuss later.

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
	pragma final = true
}
```

Note that pragmas apply to the thing that they are inside.
At the top level of a file that means the current module.

## Pattern Matching

## Effects

Effects are one of Ranni's core features, allowing a great deal of functionality
to be pushed out into effects that only happen at runtime.
There are three major ideas when it comes to Effects in Ranni:

1. The compiler cannot reduce effects. When encountered it will simply stop
	reducing the expression any further, leaving it expanded and dependent on the
	effect. This also means that we can statically determine and enforce the order
	of effects when they do run.

2. Effects themselves are Capability Types. In order to read a file you need to
	call the `read_file` method on an `FS` struct. All capabilities stem from a
	single `Root` that is passed into the program's entry point.

3. In order to make passing effects more ergonomic functions support a system of
	*dynamic arguments* that are automatically passed between them. Functions
	can automatically pass effects to callees and enforce that they are passed
	all the effects their callees require.

That was a lot so let's see it in action. Let's define a function `piper` that
takes in a file and a path and copies the file to the path.
```rs
# We import the filesystem module from the standard library
let fs = import "pkg:std/fs"

# Effect arguments are written in angle brackets before the arguments
let piper = fn <fs.FS>(in_file: fs.File, out_path: fs.Path) {
	# Effects are available with the same name as their type
	let data: [Byte] = FS.read_all(in_file)

	let out_file = FS.open(out_path, fs.Mode.Write)
	let writer = FS.write_all(out_file, data)
}
```

And to illustrate how effects propagate let's add an indirect function
```rs
# Inferred to have the fs.FS effect
# We don't need to specify that we're passing fs.FS to piper
let indirect = fn => piper($0, $1)
```
If a function takes an effect it can implicitly pass that effect to callees.
If not then it must be passed manually (see below).

Now to get the effect we need an entry point function, which we'll call `main`.
```rs
# Main takes the Root capability
let main = fn <std.Root> () {
	# The Root effect contails all the others
	let FS = Root.FS

	# Set some things up
	let out_path = fs.Path("/tmp/test/output")
	let in_path = fs.Path("/tmp/test/input")

	let in_file = FS.open(in_path, fs.Mode.Read)

	# FS has to be passed in manually because it was not part of main
	indirect<FS>(in_file, out_path)
}
```

## Mutability

Mutability is done using the `std.mem.Alloc` effect.
Operations on mutable collections require this effect.
```rs
let mem = import "pkg:std/mem"

let foo = fn <mem.Alloc> {
	let mut_array = mem.MutArray(Int) # Requires Alloc
	mut_array.insert(3) # Also requires Alloc
	mut_array.insert(4)

	# mut_array now looks like [3, 4]

	# Still requires Alloc since the value of mut_array depends on Alloc
	mut_array.get(1)
}
```

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

## Type Checking

### Type Inference

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
