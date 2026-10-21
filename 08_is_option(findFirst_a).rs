
fn main() {
    let index = find_first_a(String::from("krish"));

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
}

fn find_first_a(para: String) -> Option<i32> {
    for (index, char) in para.chars().enumerate() {
        if char == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}


/* option is like this  enum function inside rust library (here as a summary)

enum Option <T> {
    Some(T),
    None,
}

*/
