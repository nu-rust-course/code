# Lecture 2: A First Rust Program

First we saw how to create a new Rust project with the [`cargo new`]
command.

Then we then wrote [a program][main.rs] to read a Fahrenheit temperature
from [`stdin`], convert it to Celsius, and print the result to
[`stdout`]. The conversion itself is defined in [a library][lib.rs] that
is conceptually separate from (and [imported by][use lib]) the main
program. The library also demonstrates how special triple-slash *doc
comments* can be used to [attach documentation to function][doc
comment], and how we can write unit tests in [a conditionally-compiled
submodule][unit tests]. In order to understand a bit about Rust IO and
error handling, you may want to read through the [`read_input`] function
carefully; make sure you understand why we are calling [`unwrap`] and
[`expect`] on various [`Result`]s.

[main.rs]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/main.rs

[lib.rs]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/lib.rs

[use lib]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/main.rs#L4

[doc comment]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/lib.rs#L3-L10

[unit tests]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/lib.rs#L16-L33

[`read_input`]:
    https://github.com/nu-rust-course/code/blob/master/02-convert/src/lib.rs#L3-L10

[`stdin`]:
    https://doc.rust-lang.org/std/io/fn.stdin.html

[`stdout`]:
    https://doc.rust-lang.org/std/io/fn.stdout.html

[`Result`]:
    https://doc.rust-lang.org/std/result/enum.Result.html

[`unwrap`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap

[`expect`]:
    https://doc.rust-lang.org/std/result/enum.Result.html#method.expect

[`cargo new`]:
    https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html
