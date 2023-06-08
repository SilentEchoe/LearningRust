fn main() {
    let calss2 = Class::ClassTwo(Student::StudentOne);

    if let cass = Class::ClassTwo(Student::StudentOne) {
        println!("ClassTwo");
    }
}

enum Student {
    StudentOne,
    StudentTwo,
}

enum Class {
    ClassOne,
    ClassTwo(Student),
}
