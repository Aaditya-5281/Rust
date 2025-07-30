use crate::enums;

pub enum Names{
    Aaditya,
    MaheshBabu,
    RamCharan,
}


pub fn whatIsYourName(name: Names) -> &'static str {
    match name {
        Names::Aaditya => "Your name is Aaditya",
        Names::MaheshBabu => "Your name is Mahesh Babu",
        Names::RamCharan => "Your name is Ram Charan",
    }
}   

