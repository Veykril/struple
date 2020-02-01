use struple::Struple;

#[derive(Struple)]
struct Generic<A, B, C, D> {
    bar: A,
    baz: B,
    quux: D,
    qux: C,
}

#[derive(Struple)]
struct GenericLifetimes<'a, A, B, C> {
    bar: A,
    baz: B,
    qux: &'a C,
}

#[derive(Struple)]
struct Named {
    bar: i32,
    baz: u64,
    qux: String,
}

#[derive(Struple)]
struct Tuple(i32, u64, String);

#[derive(Struple)]
struct Unit;

#[test]
fn test_named_impl() {
    let foo = Named::from_tuple((3, 5, "bar".into()));
    assert_eq!(foo.into_tuple(), (3, 5, "bar".into()));
}

#[test]
fn test_tuple_impl() {
    let foo = Tuple::from_tuple((3, 5, "bar".into()));
    assert_eq!(foo.into_tuple(), (3, 5, "bar".into()));
}

#[test]
fn test_unit_impl() {
    let foo = Unit::from_tuple(());
    assert_eq!(foo.into_tuple(), ());
}

#[test]
fn test_generic_impl() {
    let foo = Generic::from_tuple((3, 5, String::from("bar"), "baz"));
    assert_eq!(foo.into_tuple(), (3, 5, "bar".into(), "baz"));
}

#[test]
fn test_generic_lifetimes_impl() {
    let bar = 0i32;
    let foo = GenericLifetimes::from_tuple((3, 5, &bar));
    assert_eq!(foo.into_tuple(), (3, 5, &bar));
}
