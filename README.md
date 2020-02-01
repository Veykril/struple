Struple
=======

Derive conversion routines from and to tuples for your structs.

## Basic example

```rust
use struple::Struple

#[derive(Struple)]
struct Foo(String, u64, f32);

fn main() {
    let string = "hello";
    let int = 3u64;
    let float = 0.5f32;

    let foo = Foo::from_tuple((string, int, float));
    let (string, int, float) = foo.into_tuple(); 
}
```

## Use cases

The main reason as to why I made this crate is because of [nom](https://github.com/Geal/nom) and it's [tuple parser combinator](https://docs.rs/nom/5/nom/sequence/fn.tuple.html). This combinator allows easy chaining of serveral parsers in sequence but has the downside that you gotta destructure the tuple manually to put the data into the corresponding struct. This is cumbersome and bloats the code(at least in my opinion). With this one can just derive a tuple constructor for the struct and pass that constructor to the map function to map the tuple to the struct.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
