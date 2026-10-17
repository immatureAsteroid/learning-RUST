fn main() {
    let var1 = fib(5); 
    println!("{}", var1);

}

fn fib(num : u32) -> u32 { //here unsigned liye hain dono hi 

    let mut first = 0;  // mut means mutable/non constent in rust [because default constent hi hai]
    let mut second = 1;
    if num ==0 {
         return first;
    }
    if num ==1 {
         return second; 
        }

    for _ in 0..(num-1){
        let temp= second;
        second = second+first ;
        first=temp;

    }
 return second;

}
// in fibinacci : 5th place par 5 hi value hai if staring from zero
