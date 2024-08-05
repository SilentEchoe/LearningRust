
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        match e {
            MyEnum::Foo => count += 1,
            _ => {}
        }
    }

    assert_eq!(count, 2);
}