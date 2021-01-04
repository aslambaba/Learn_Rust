// Traits, Default Traits, Multiple Methods

#[derive(Debug)]
struct Student{
    Firstname: String,
    Lastname: String,
}
#[derive(Debug)]
struct Teacher{
    Firstname: String,
    Lastname: String,
}

pub trait FullName{
    fn fulnam(&self)->String{
        format!("Invalid Code")
    }
    fn lastnam(&self)->String{
        format!("Invalid Code")
    }
}

impl FullName for Student{}
impl FullName for Teacher{
    fn fulnam(&self)->String{
        format!("{} {}", self.Firstname, self.Lastname)
    }
    fn lastnam(&self)->String{
        format!("{}", self.Lastname)
    }
}
fn main() {

    // Simple Traits

    let StudentOne = Student{
        Firstname: String::from("Aslam"),
        Lastname: String::from("Baba"),
    };
    
    println!("Your Full Name is: {}", StudentOne.fulnam());
    
    // Traits, Defualt traits, Multiple Methods

    let Aslam = Student{
        Firstname: String::from("Aslam"),
        Lastname: String::from("Sarfraz"),
    };
    let Laila = Teacher{
        Firstname: String::from("Laila"),
        Lastname: String::from("Khalid"),
    };
    
    println!("{}",Aslam.fulnam());
    println!("{}",Laila.lastnam());
}