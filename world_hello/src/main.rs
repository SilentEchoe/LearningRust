fn main() {
    let a = MyEnum::A;
    assert!(matches!(a, MyEnum::A));

    let age = Some(3);
    println!("age: {:?}", age);
    if let Some(age) = age {
        println!("age: {:?}", age);
    }
    println!("age: {:?}", age);
}

enum MyEnum {
    A,
    B,
    C,
}
