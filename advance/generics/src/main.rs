// In this Part We use Generics 
// - As a Functions
// - As a Custom Type (Struct,Enum)
// - As a Mixup of functions,methods
// - Custom programs

#[derive(Debug)]

struct Person <T> {
    name:T,
    profession:T,
}
#[derive(Debug)]
struct MixupProgram<T,V>{
    name: T,
    age: V,
}

fn main() {

    // As a Function

    GenericsFunction("qwerty91r");
    GenericsFunction(1223300);
    
    fn GenericsFunction <T: std::fmt::Display> (password:T){
        println!("Your Password is {}", password);
    }

    // As a Custom Type Struct

    let Aslam = Person{
        name: "Aslam Baba",
        profession: "Mern Stack Developer",
    }
    println!("{:#?}", Aslam);

    // Using As a method

    impl<T: std::fmt::Display> Person<T>{
        fn PrintName(&self){
            println!("Your Name is: {}", self.name);
        }
    }

    Aslam.PrintName();

    // Mixup Program

    impl<T,V: std::fmt::Display> MixupProgram<T,V>{
        fn mix <C:std::fmt::Display,B:std::fmt::Display>(&self, other: MixupProgram<C, B>){
            println!("{}",other.name);
        }
    }

    let Rehan = MixupProgram{
        name: "Muhammad Rehan",
        age: 20,
    };
    let Shayan = MixupProgram{
        name: "Shayan",
        age: 18,
    };
    Rehan.mix(Rehan2);
}
