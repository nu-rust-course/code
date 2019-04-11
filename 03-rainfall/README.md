# Lecture 3: Rainfall

This is an implementation of [rainfall] in Rust. The design factors
input, computation, and output into separate functions. Instead of
reading from stdin and writing to stdout, the IO functions are
parameterized to support unit testing:

  - Function [`read_measurements`] can read from any type that
    implements trait [`Read`] (*e.g.,* [`Stdin`], [`File`], or
    [`&[u8]`][std::slice]).

  - Function [`write_output`] can write to any type that implements
    trait [`Write`] (*e.g.,* [`Stdout`], [`File`], or
    [`&mut Vec<u8>`][`Vec`]).

Additionally, instead of panicking on errors, the IO functions report
errors by returning [`std::io::Result`]s.

## Iterators and error propagation

You may find [`read_measurements`] difficult to read, as it’s written in
a functional style using [`Iterator`] tranformers. So first let's
consider two simpler versions of the function.

Function [`read_measurements0`] also uses iterator tranformers, but
because it punts on error handling, it may be easier to understand. In
particular, it:

 1. [creates an iterator][creates an iterator0] over the lines of the
    input,
 2. [checks for errors][checks for errors0] and panics if it encounters
    one,
 3. [trucates the stream][trucates the stream0] if it sees the
    termination code `"999"` (where `|…| …` is Rust syntax for a
    [lambda], with the parameters between the pipes and the body after),
 4. [attempts to parse][attempts to parse0] each line into an `f64`,
    filtering parsing failures out of the stream,
 5. [filters out][filters out0] negative readings, and finally
 6. [collects][collects0] the remaining readings into an `Vec<f64>`.

Step 6 may seem kind of magical, because the [`Iterator::collect`]
method can accumulate the values of an iterator into [a variety of
different collection types][FromIterator implementors]. For example,
the item `impl FromIterator<char> for String` means that an iterator
over characters can be collected into a string, whereas the item
`impl FromIterator<String> for String` means that an iterator over
strings can also be collected into a string. The `impl` used by this
step 6 is `impl<T> FromIterator for Vec<T>`.

Next, function [`read_measurements1`] propagates errors to its caller
rather than panicking, but rather than using the functional/iterator
style, it’s written in an imperative style using a mutable vector
variable, a `for` loop, a `break` statement, and several `if`s. This is
close to how you’d write it in C++. Note that `let line = line?;` checks
whether `line` (a `Result`) is an error or okay. If it’s an error then
the function returns immediately, propagating the error; but if `line`
is okay then `?` extracts the `String` from it and binds `line` (a
different variable that happens to have the same name) to that.

The imperative implementation [`read_measurements1`] is correct, and you
don’t need to be able to write fancy iterator transformer chains to
write excellent Rust. You should, though, at least be able to read both
ways of expressing this kind of algorithm. So let’s return to
[`read_measurements`] and read through it step by step. It:

 1. [creates an iterator] over the lines of the input,
 2. [trucates the stream] if it sees the termination code `"999"`,
 3. [attempts to parse] each line into an `f64`, filtering it out of the
    stream when parsing fails,
 4. [filters out] negative readings, and finally
 5. [collects] the remaining readings into an `io::Result<Vec<f64>>`.

This time, step 5 is particularly interesting. As in the other
implementations, the stream of lines returned by [`BufRead::lines`] is
an iterator not over `String`s but over `io::Result<String>`s; but
unlike in [`read_measurements0`], we don’t bail out on errors. Instead,
steps 2–4 all have to deal with the possibility of errors, which is why
steps 2 and 3 use [`Result::map`] to work on `Ok` results while passing
`Err` results through unchanged, and why step 4 uses
[`Result::unwrap_or`] to map errors to a number that the filter
predicate accepts.

Thus, coming out of step 4 and into step 5 is a stream of
`io::Result<f64>`s, and [`Iterator::collect`] must turn an iterator over
`io::Result<f64>`s turn into an `io::Result<Vec<f64>>`. What does this
mean? If every `io::Result<f64>` in the stream is `Ok` then it returns
`Ok` of a vector containing all the `f64`s, but if it ever encounters
`Err` of some `io::Error` `e` then it returns `Err(e)` immediately as
well. Here is the `impl` logic:

```rust
impl<T, E, C> FromIterator<Result<T, E>> for Result<C, E>
where
    C: FromIterator<T>
```

That is:

  - For any types `T` (the element), `E` (the error), and `C` (the
    container),
  - if an iterator over `T`s can be collected into a `C`,
  - then an iterator over `Result<T, E>`s can be collected into
    a `Result<C, E>`.

Noting that [`io::Result<A>`][`std::io::Result`] is a synonym for
`Result<A, io::Error>`, we can see that [step 5][collects] uses
the aforementioned `impl` with `T = f64`, `E = io::Error`, and
`C = Vec<f64>`.

## Testing IO

Making our IO functions generic over the [`Read`] and [`Write`] traits
means that it’s easy to test `read_measurements` and `write_output` from
within Rust’s built-in unit testing framework.

In fact occasionally we might write our whole program as [a function
from input to output][`transform`]. (**Don’t do this on your homework,**
because all your homework programs are intended to be interactive.)

In any case, parameterizing our functions this way lets us write
assertions that:

  - [`read_measurements` parses] a particular input (given as a string
    literal) into a particular internal representation;

  - [`write_output` unparses] a particular internal representation into
    a particular output (given as a string literal); and

  - [`transform` transforms] a particular input into a particular output
    (both given as string literals).

When writing tests that require some special setup or comparison, it’s
not very nice to repeat that code. It’s much nicer to abstract the
boilerplate into a function like [`assert_read`], [`assert_write`], or
[`assert_transform`], and then express each of your test cases in terms
of your new assertion. Read [`assert_transform`] carefully to see how
it:

  1. [creates an empty vector] of bytes to use as a mock [`Write`],
  2. [views a string as a byte array] to use as a mock [`Read`],
  3. [attempts to convert] the `Vec<u8>` output [into a UTF-8
     string][`String::from_utf8`], failing the test if it can’t, and
     finally
  4. [asserts that the output] was what we expected.

[rainfall]:
    http://users.eecs.northwestern.edu/~jesse/course/eecs396rust/labs/rainfall.txt

[`transform`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L71-L74

[`read_measurements0`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L116-L123

[`read_measurements1`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L127-L145

[`read_measurements`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L148-L154

[`write_output`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L233-L241

[creates an iterator0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L117

[checks for errors0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L118

[trucates the stream0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L119

[attempts to parse0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L120

[filters out0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L121

[collects0]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L122

[creates an iterator]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L149

[trucates the stream]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L150

[attempts to parse]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L151

[filters out]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L152

[collects]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L153

[`assert_read`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L181-L184:

[`assert_write`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L263-L269

[`assert_transform`]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L99-L104

[`read_measurements` parses]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L161-L178

[`write_output` unparses]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L247-L260

[`transform` transforms]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L80-L97

[creates an empty vector]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L100

[views a string as a byte array]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L101

[attempts to convert]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L102

[asserts that the output]:
    https://github.com/nu-rust-course/code/blob/master/03-rainfall/src/main.rs#L103

[lambda]:
    https://doc.rust-lang.org/beta/book/ch13-01-closures.html

[std::slice]:
    https://doc.rust-lang.org/std/primitive.slice.html

[`Vec`]:
    https://doc.rust-lang.org/std/vec/struct.Vec.html

[`Iterator`]:
  https://doc.rust-lang.org/std/iter/trait.Iterator.html

[`Iterator::collect`]:
  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect

[`Read`]:
    https://doc.rust-lang.org/std/io/trait.Read.html

[`Write`]:
    https://doc.rust-lang.org/std/io/trait.Write.html

[`BufRead::lines`]:
    https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines

[`Stdin`]:
    https://doc.rust-lang.org/std/io/struct.Stdin.html

[`Stdout`]:
    https://doc.rust-lang.org/std/io/struct.Stdout.html

[`File`]:
    https://doc.rust-lang.org/std/io/struct.File.html

[`std::io::Result`]:
    https://doc.rust-lang.org/std/io/enum.Result.html

[`stdin`]:
    https://doc.rust-lang.org/std/io/fn.stdin.html

[`stdout`]:
    https://doc.rust-lang.org/std/io/fn.stdout.html

[`Result`]:
    https://doc.rust-lang.org/std/result/enum.Result.html

[`Result::map`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.map

[`Result::unwrap_or`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or

[`unwrap`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap

[`expect`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.expect

[FromIterator implementors]:
    https://doc.rust-lang.org/std/iter/trait.FromIterator.html#implementors

[`String::from_utf8`]:
    https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8

