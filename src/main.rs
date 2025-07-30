mod my_struct; 
mod enums;

use enums::{Names,whatIsYourName};

use my_struct::User; 

// Ownership
fn main() {
let mut boyfriend = String::from("Rose");
boyfriend = takes_ownership(boyfriend);  
println!("{}", boyfriend); 


// //Borrowing
let owner = String::from("The Data"); 
    let reader = &owner;                   
    println!("{}", reader);                
    println!("{}", owner); 

// // Instance of struct

let user = User {
    userName: String::from("Rose"),
    age: 21,
    email: String::from("rose@gmail.com"),
};

println!("Name: {}, Age : {}, Email {}",user.userName,user.age,user.email );


// Option 1: Basic Acessing
let userStruct = User {
        userName: String::from("Aaditya"),
        age: 21,
        email: String::from("aaditya@gmail.com"),
    };

    println!("The Name is {} and Age is {}. Email is {}", userStruct.userName, userStruct.age, userStruct.email);

    // Option 2: Advanced Acessing (Check  my_struct)
    let u= User::userDetails("Aaditya", 20, "aaditya@gmail.com");
    println!("The Name is {} and Age is {}. Email is {}", u.userName, u.age, u.email);

}

// Enums 
// fn main(){
//     let myName = Names::Aaditya;
//     let what_is_your_name = whatIsYourName(myName);
//     println!("{}",what_is_your_name)
// }



fn takes_ownership(bestie: String) -> String {
println!("{}", bestie); // Hangs out with new guy
bestie // Returns to ex

}








  





