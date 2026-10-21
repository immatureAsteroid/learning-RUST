fn main() {
    let var1 = is_even(12);
    println!("{}", var1);

}

fn is_even(num : i32) -> bool {   //i32 signed hai
    if num%2==0 {
        return true;

    } else {
        return false;}
}
