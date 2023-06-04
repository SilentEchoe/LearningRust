fn main() {
    let array: [String; 8] = std::array::from_fn(|i| String::from("rust is good!"));

    println!("{:#?}", array);
}
