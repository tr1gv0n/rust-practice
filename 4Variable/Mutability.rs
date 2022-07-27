fn main() {
    let _immutable_binging = 1;
    let mut mutable_binging = 1;

    println!("Before mutation: {}", mutable_binging);

    //Ok
    mutable_binging += 1;

    println!("After mutation: {}", mutable_binging);

    //Error
    _immutable_binging += 1;
}
