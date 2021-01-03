fn main() {
    println!("Hello, world!");

    // Stack, Heap Ownership Concepts
    stack_heap_concepts();

    // Custom DataType - Struct

    struct_dt();

    area_calculation();
}

fn stack_heap_concepts(){

    // Here is Use Heap Variable or we say String Litrel where can i expand the 
    // size of my variable.
    let mut name = String::from("My Name is Aslam");
    name.push_str(" And my age is 20");
        println!("{}", name);

    // If i use Stack Variable so i can chaneg the value but not increase the size 
    // of it
    
    // RUN THIS CODE AND IT WILL NOT RUN

    // let mut name_stack = "My Name is Aslam";
    // name_stack.push_str(" And my age is 20");
    //     println!("{}", name_stack);

    let name = String::from("Hello");
    // let name2 = name; -> We Can not copy heap data because heap move the  value.
    // But you can clone it but it is not recomended beacuse it incease the size of heap
        let name2 = name.clone(); 
    println!("{} {}", name,name2);


    // === OwnerShip In Functions ==== //

    let name = String::from("Aslam");
    take_name(name);
    // println!("{}", name); -> Now we can not use name value in this function
    // because now heap move value to take_name();
    fn take_name(x: String){
        println!("Your Name is {}", x)
    }

    // You can print value in main function with return multiples values

    let father_name = String::from("Nusrat");
    let (return_father_name,agee) = take_father_name(father_name);
    println!("Your Father Name is {} and his age is {}", return_father_name,agee);
    fn take_father_name(y: String)-> (String,u64) {
        let age = 20;
        (y,age)
        
    }

    // === Borwoing Concepts === //

    // Here i borrow college variable address and pass on function and in function
    // i also declear that we borow the value/address of college variable.
    // And we use mut beacuse now the ownership is owned by college beacuse we just
    // pass the address so mut is used averywhere to define that we borrow value not
    // get ownership.
    
    let mut college = String::from("PGC");
    get_college(&mut college);

    fn get_college(mycollege: &mut String){
        mycollege.push_str(" -> Punjab Group of College");
        println!("{}", mycollege);
    }
    
}

#[derive(Debug)]
struct Friends {
    name: String,
    age: u64,
    college: String,
}
#[derive(Debug)]
struct Student {
    name: String,
    age: u64,
    gender: String,
    class: String,
    college: String,
}
fn struct_dt(){

    // Now We Define Custome Datatype Using Struct Where we store multiple values 
    // in a single variable with different datatype also with name.
    
    // As you see we define Sturck outside the function and call it with the name
    // happy
    let mut Happy = Friends {
        name: String::from("Shahzad"),
        age: 20,
        college: String::from("Post Graduate"),
    };
    // Change value.
    Happy.name = String::from("Shahzad Khan");
    println!("{:#?}", Happy);

    let call = call_struck_fn(String::from("Uzair"), 22);
    println!("{:#?}", call);

    // Also Remember when we return value from function we also declear struck
    // in return value as a datatype.

    fn call_struck_fn(name: String, age: u64) -> Friends{
        Friends{
            name: name,
            age: age,
            college: String::from("VTI"),
        }
    }



    let studentOne = Student {
        name: String::from("Aslam"),
        age: 20,
        gender: String::from("Male"),
        class: String::from("ICS"),
        college:String::from("Superior College BWP"),
    };

    let studentTwo = Student {
        name: String::from("Abdul Mateen"),
        age: 19,
        // Advance santax when you use another struck values.
        gender: studentOne.gender,
        ..studentOne
    };

    println!("{:#?}", studentTwo  );
    
    // We can also use Tuple in struct in which we can not provide name just values
    // as you see
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
}

#[derive(Debug)]

struct Dimention {
    height: u64,
    width: u64,
}

// This is the Example of Struct
fn area_calculation(){
    let data = Dimention{
        height: 10,
        width: 2,
    };

    let pri = ac(data);
    println!("{}", pri);

    fn ac(x: Dimention) -> u64{
        return x.height*x.width;
    }
}