fn main() {
    let mut _mutable_interger = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_interger = _mutable_interger;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_interger = 50;
        // `_mutable_integer` goes out of scope
    }

    _mutable_interger = 3
}
