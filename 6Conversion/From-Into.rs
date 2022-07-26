use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // For example we can easily convert a str into a String

    /*
     *      let my_str = "hello";
     *      let my_string = String::from(my_str);
     */

    //From
    /*
     *    let num = Number::from(30);
     *    println!("My number is {:?}", num);
     */

    //Into
    let int = 5;

    let num: Number = int.into();
    println!("My number is {:?}", num);
}
