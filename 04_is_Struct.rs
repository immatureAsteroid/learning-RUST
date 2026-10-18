fn main() {
    let ram = User {
        first_name: String::from("JAI"),
        last_name: String::from("RAM"),
        age: 45,
    };
    println!("{} {}", ram.first_name , ram.last_name); 
    /*khali ("{}", ram) se saare feild print nahi honge because print does not know how to do that,
     thatswhy 1 or more than 1 feild needed */

}

struct User {
    first_name: String,
    last_name: String,
    age: i32,
}
