use struple::Struple;

#[derive(Struple)]
struct Unary(i32);

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

#[derive(Struple)]
#[cfg(feature = "big-tuples")]
struct Huge {
    _1: i32,
    _2: i32,
    _3: i32,
    _4: i32,
    _5: i32,
    _6: i32,
    _7: i32,
    _8: i32,
    _9: i32,
    _10: i32,
    _11: i32,
    _12: i32,
    _13: i32,
    _14: i32,
    _15: i32,
    _16: i32,
    _17: i32,
    _18: i32,
    _19: i32,
    _20: i32,
    _21: i32,
    _22: i32,
    _23: i32,
    _24: i32,
    _25: i32,
    _26: i32,
    _27: i32,
    _28: i32,
    _29: i32,
    _30: i32,
    _31: i32,
    _32: i32,
    _33: i32,
    _34: i32,
    _35: i32,
    _36: i32,
    _37: i32,
    _38: i32,
    _39: i32,
    _40: i32,
    _41: i32,
    _42: i32,
    _43: i32,
    _44: i32,
    _45: i32,
    _46: i32,
    _47: i32,
    _48: i32,
    _49: i32,
    _50: i32,
    _51: i32,
    _52: i32,
    _53: i32,
    _54: i32,
    _55: i32,
    _56: i32,
    _57: i32,
    _58: i32,
    _59: i32,
    _60: i32,
    _61: i32,
    _62: i32,
    _63: i32,
    _64: i32,
    _65: i32,
    _66: i32,
    _67: i32,
    _68: i32,
    _69: i32,
    _70: i32,
    _71: i32,
    _72: i32,
    _73: i32,
    _74: i32,
    _75: i32,
    _76: i32,
    _77: i32,
    _78: i32,
    _79: i32,
    _80: i32,
    _81: i32,
    _82: i32,
    _83: i32,
    _84: i32,
    _85: i32,
    _86: i32,
    _87: i32,
    _88: i32,
    _89: i32,
    _90: i32,
    _91: i32,
    _92: i32,
    _93: i32,
    _94: i32,
    _95: i32,
    _96: i32,
    _97: i32,
    _98: i32,
    _99: i32,
    _100: i32,
}

#[test]
fn test_unary_impl() {
    let foo = Unary::from_tuple((3,));
    assert_eq!(foo.into_tuple(), (3,));
}

#[test]
#[cfg(feature = "big-tuples")]
fn test_huge_impl() {
    let foo = Huge::from_tuple((
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
        95, 96, 97, 98, 99, 100,
    ));
    let bar = foo.into_tuple();

    // Note) Tuple does not implement PartialEq on Tuples whose arity is greater than 12
    assert_eq!(bar.0, 1);
    assert_eq!(bar.99, 100);
}

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
