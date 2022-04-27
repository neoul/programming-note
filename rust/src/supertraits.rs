
pub fn example() {
    trait Person {
        fn name(&self) -> String;
    }
    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    fn student_greeting(student: &dyn Student) -> String {
        format!("hello, I'm {} and attend {}", student.name(), student.university())
    }
    
    struct Boy {
        name: String,
        university: String
    }
    impl Person for Boy {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl Student for Boy {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    let b = &Boy{
        name: String::from("willing"), 
        university: String::from("sangmyung")
    };
    let greeting = student_greeting(b);
    println!("{}", greeting);
}