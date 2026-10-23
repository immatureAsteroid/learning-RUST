/*code ko chalane ke liye Cargo.toml mein chrono dependency honi chahiye
thats why in ternimal >> cargo add chrono
*/
use chrono::{Local, Utc}; // chrono crate se Utc and Local  ko import kiya, jo UTC time deta hai.

fn main() { 
    let var1 = Utc::now(); // Abhi ka current UTC date aur time 
    let var2 = Local::now(); // System ka current local date aur time lega

    println!("local system time is {}", var2); 
    println!("current time is {}", var1); 
} 
