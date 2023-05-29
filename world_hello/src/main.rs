fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
