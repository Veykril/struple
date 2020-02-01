use struple::Struple;

#[derive(Struple)]
enum Foo {
    Bar(i32, u64, String),
    Baz,
}

fn main() {}
