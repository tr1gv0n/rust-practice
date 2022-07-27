#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;

    let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("-1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 mod 256 is :{}", 1000 % 256);

    println!("128 as a i8 is :{}", 128 as i8);

    println!("128 as a i16 is :{}", 128 as i16);

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
