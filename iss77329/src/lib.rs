trait Foo<T> {}
fn bug() -> impl Foo<[(); |_: ()| {}]> {}
