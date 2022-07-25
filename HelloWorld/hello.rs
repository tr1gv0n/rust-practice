// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // println!("Hello World!");

    // println!("I'm a Rustacean!");

    /*
     * Block comments
     * Windows: Shift + Alt + A.
     * Mac: Shift + Option + A.
     */
    // let x = 5 + /* 90 + */ 5;
    // println!("Is `x` 10 or 100? x = {}", x);

    // println!("{} days", 31);

    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // println!(
    //     "{subject} {verb} {object}",
    //     object = "the lazy dog",
    //     subject = "the quick brown fox",
    //     verb = "jumps over"
    // );

    // println!("Base 10 repr: {}", 69420);
    // println!("Base 2  (binary) repr: {:b}", 69420);
    // println!("Base 8  (octal) repr: {:o}", 69420);
    println!("Base 16  (hexadecimal) repr: {0:0>2x} {1:x}", 0, 254);
    // println!("Base 16  (hexadecimal) repr: {:X}", 69420);

    // println!("{number:>5}", number = 1);
    // println!("{number:0>5}", number = 1);
    // println!("{number:0>width$}", number = 1, width = 5);

    // #[derive(Debug)]
    // struct Structure(i32);

    // println!("This struct `{:?}` won't print...", Structure(3));

    // let number: f64 = 1.0;
    // let width: usize = 6;
    // println!("{number:>width$}");

    // println!("{:?} months in a year.", 12);

    // println!(
    //     "{1:?} {0:?} is the {actor:?} name.",
    //     "Slater",
    //     "Christian",
    //     actor = "actor's"
    // );

    // println!("Noe {:?} will print!", Structure(3));

    // println!("Now {:?} will print!", Deep(Structure(7)));

    // let name = "Peter";
    // let age = 27;

    // let peter = Person { name, age };

    // println!("{:#?}", peter);
}
