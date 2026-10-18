fn main() {
    let var1 = String::from("Balloon"); 
    println!("the length of the string is: {}", get_string_length(var1));

}

fn get_string_length(str1: String) -> usize {
    str1.chars().count()      // ye implicit return hai 
    //return str1.char().count();   //bhi kar sakte hain
}
