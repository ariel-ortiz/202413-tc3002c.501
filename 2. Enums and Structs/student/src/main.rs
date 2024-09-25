struct Student {
    name: String,
    id: u32,
    gpa: f64
}

fn main() {
    let mut s1 = Student::new("Juan".to_string(), 123, 3.5);
    let mut s2 = Student::new("Maria".to_string(), 199, 4.0);

    println!("{}, {}, {}", s1.name, s1.id, s1.gpa);
    println!("{}, {}, {}", s2.name, s2.id, s2.gpa);

    s1.say_hi();
    s2.say_hi();

    s1.set_gpa(3.6);
    s2.set_gpa(3.9);

    println!("{}", s1.get_gpa());
    println!("{}", s2.get_gpa());
}

impl Student {

    fn new(name: String, id: u32, gpa: f64) -> Self {
        Self {name, id, gpa}
    }

    fn say_hi(&self) {
        println!("{} (id: {}) says hi!", self.name, self.id);
    }

    fn get_gpa(&self) -> f64 {
        self.gpa
    }

    fn set_gpa(&mut self, new_gpa: f64) {
        self.gpa = new_gpa;
    }
}
