// Ownership
fn main() {
let mut boyfriend = String::from("Rose");
boyfriend = takes_ownership(boyfriend);  
println!("{}", boyfriend); 


//Borrowing
let owner = String::from("The Data"); 
    let reader = &owner;                   
    println!("{}", reader);                
    println!("{}", owner); 

// Instance of struct

let user = User {
    userName: String::from("Rose"),
    age: 21,
    email: String::from("rose@gmail.com"),
};

print!("Name: {}, Age : {}, Email {}",user.userName,user.age,user.email )


}


fn takes_ownership(bestie: String) -> String {
println!("{}", bestie); // Hangs out with new guy
bestie // Returns to ex

}


// Struct

struct  User{
    userName : String,
    age : u32,
    email : String
}


  





