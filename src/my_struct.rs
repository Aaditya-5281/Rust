pub struct User {
    pub  userName: String,
    pub  age: u32,
    pub  email: String,
}

impl User {
    pub fn userDetails(name : &str, age :u32 , email : &str) -> Self {

        Self{
            userName : name.to_string(),
            age,
            email : email.to_string()
        }
    }
}

