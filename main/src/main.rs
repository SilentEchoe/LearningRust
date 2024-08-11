
trait Person {
    fn name(&self) -> String;
}

// Person 是 Student 的 supertrait .
// 实现 Student 需要同时实现 Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) 是 Programmer
// 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}

// 为 CSStudent 实现所需的特征
impl Programmer for CSStudent{
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl Person for CSStudent {
    fn name(&self) -> String {
       self.name.clone()
    }
}

impl Student for CSStudent{
    fn university(&self) -> String{
        self.university.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String{
        self.git_username.clone()
    }
}

fn main() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    // 填空
    println!("{}", comp_sci_student_greeting(&student));
}