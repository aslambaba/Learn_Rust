// Traits, Default Traits, Multiple Methods, Trait Bounds

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

// Traits Bound

// Simple Trait Bound
pub fn notify<T: FullName>(Item: T){
    println!("{}", Item.fulnam());
}

// This trait bound is use when we strickly use same datatypes
pub fn notify2<T: FullName>(ItemOne: T , itemTwo: T){
    println!("{} and {}", ItemOne.fulnam(), itemTwo.lastnam());
}

// This Trait Bound is use when we want to pass different datatypes
pub fn notify3(itemOne: impl FullNamem, itemTwo: impl FullName){
    println!("{} and {}", itemOne.fulnam, itemTwo.lastnam)
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

    // Simple Using trait bound
    notify(Aslam);
    notify(Laila);
}
