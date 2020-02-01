use struple::Struple;

#[derive(Struple)]
union Foo {
    foo: i32,
    bar: u64,
}

fn main() {}
