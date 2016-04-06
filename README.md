PostgreSQL Allocator for Rust
====

This is an implementation of a [*custom allocator*][custom-alloc] for the [Rust programming
language][rust] which allows Rust to make use of PostgreSQL's memory allocator.

Why on earth would you want to do this?

Well, it allows you to write functions that interact with Postgres' internals **in Rust**. So far,
this allocator has been used to implement a [custom Postgres type][create-type] for email addresses,
with more to come!

## Usage

At the moment, you have to be on the nightly builds of the Rust compiler in order to use
custom allocators. If you're using nightly, then switching all your allocations to the
Postgres allocator is as simple as this:

```rust
extern crate postgres_alloc;
```

You'll also probably want to link your Rust library in with some C wrapper functions,
so that you can use Postgres' plethora of macros. A blog post about this is planned - soon!

See also: [Postgres docs on C-language functions][cfuncs]

## Acknowledgements

This allocator was written by Michael Sproul and Angus Thomsen (@AKST) for the COMP9315 course
on database internals at UNSW. Thanks to John Shepard for entertaining our desire to use Rust!

[rust]: https://www.rust-lang.org/
[custom-alloc]: https://doc.rust-lang.org/book/custom-allocators.html
[cfuncs]: http://www.postgresql.org/docs/current/static/xfunc-c.html
[create-type]: http://www.postgresql.org/docs/current/static/sql-createtype.html
