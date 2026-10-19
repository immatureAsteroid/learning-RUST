fn main() {
    let rec1= Rect {
        width: 8,
        length: 9,
    };

    println!("area is: {}",rec1.area());
    println!("increased length is: {}",rec1.inc_length(2));
    println!("debug test is: {}",Rect::debug_test()); //oject name se nahi pukar sakte, direct strcuture ka hi name.

}

struct Rect {
    width: i32,
    length: i32,
}
impl Rect {
    fn area(&self)-> i32{ //first parameter is for self pointing[self is a implicit parameter]
        self.width*self.length
    }
    fn inc_length(&self, num:i32)-> i32{ // here 2nd parameter is 1st argument when calling this method [num is a explicit parameter]
        return num + self.length;
    }
    fn debug_test()-> i32{ //no parameter means only attached to structur  & not for its Struct Instance (Object) 
        return 1;
    }
}
