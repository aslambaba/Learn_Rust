
// Simple Hello WOrld Program

fn main() {
    println!("Hello, world!");

    // Calling Functions 
    variables();

    // Data Types
    datatypes();

    // All About Funstions
    all_about_functions();

    // If Else Conditions
    ifElse_conditions();

    // Iterations or Loops
    iterations();
}

// Variables & Mutibility, Constant, Shadowing 

fn variables(){
    let temperature = 20;
    println!("This is the Templerature: {}", temperature);

    let age = 12; 

        // if you have to change the value to variable i.e variable age
        // you have to use mut before variable name so you can change the
        // value of variable 
        // For-example

    // let mut age = 12;
    // age = 13;

    println!("My Age is: {}", age);

    const VALUE_OF_PI:f64 = 3.14;

        // You have to define the data type in variable like i do, i use :f64 after name
        // of variable, and we can not use mut in const variable.

    println!("The value of PI is: {}", VALUE_OF_PI);

    let happy_age = 18;
    let happy_age = 18+1;

        // Now you see i change the value of variabe without using mut and code
        // is also compile becuase this is called shadowing means according to rust
        // variable(happy_age) in line in 37 is new and different from 1st variable.
        //  

    println!("This age of Happy is: {}", happy_age);
}

fn datatypes(){

    // There Are two Data Types
        // Static DataTypes
            // String,Int,Float,Character
        // Compound DataTypes
            // Tuple,Array

    let string_dt = "Hello";
    let int_dt = 12;
    let float_dt = 19.7;
    let char_dt = "#$";

    println!("This is String = {3} this is integer = {0} this is float = {1} this is character = {2}", int_dt,float_dt,char_dt,string_dt);

    let tuplee: (i32,f64,char) = (9,1.4,'#');
    println!("This is Tuple Where we store different type of data {:#?}", tuplee);

    let arrayy = [12,22,33];
    println!("This is Array Where we store same type of data {:#?}",arrayy);


}

fn all_about_functions(){

    // Here i call function and add paramiters with there datatype because without datatype
    // we can not pass parameters in function in rust.
    fn student_form(x:&str ,y: &str){
        println!("My Name is {} and i'm doing {} from Islamia University Bahawalpur", x,y);
    }

        // Here i call function and pass values
        student_form("aslam","BSCS");

    // Here We Call function and pass values but also return value and note that you must have
    // to define datatype of return value as you see.
    fn age_calculator(x:u32,y:u32) -> u32{
        return y-x;
    }
        // Call function where 1st parameter is the year when i born and 2nd is the current year
        let age_result = age_calculator(2000,2020);
        println!("Your age is: {}", age_result);
}

fn ifElse_conditions(){
    
    let name = "Usama";
    if name == "Aslam" {
        println!("Welcome {}", name)
    }
    else if name == "Mateen"{
        println!("Welcome {} Please wait !!", name)
    }
    else{
        println!("You are not Authorized user !! Please Register then Login");
    }
}

fn iterations(){

    let mut numb = 0;
    
    // This loop will run all the time until we break() it so we can break it.
    
    loop{
        println!("Hello From Loop");
        numb = numb + 1;
        // We break the loop with if condition as you see
        if numb == 10 {
            break();
        }
    }

    numb = 0;

    // Here i use While loop where we define condition first as you see and loop break
    // after the condition is false
    while numb < 5 {
        println!("Hello from While Loop");
        numb = numb + 1;
    }

    // Here i use For Loop where we dont have to initialize variable first we can do in
    // for loop as you see
    for a in 0..4{
        println!("Hello From For Loop {}", a);
    }

    let forarray = ["aslam","usama","Uzair","Mateen","Jhangooo","Ahmad","Happy"];

    // Here i print my array data using for loop as you seee. iter() is use to taret one value of arrray
    for friend in forarray.iter(){
        println!("My Bhai is: {}",friend);
    }
    
}