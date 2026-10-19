fn main() {
    let my_direction = Direction::East;

    testFun(my_direction); // Value function ko pass hogi (move ho jayegi)
}

enum Direction {
    North,
    East,
    West,
    South,
}

fn testFun(para: Direction) { // para Direction type ki value receive karega
    println!("hi "); // para ko use karna compulsory nahi hai, thats why error nahi aayega nahi use karne par jaroor aayega 
}
