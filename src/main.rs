use std::mem::{size_of, size_of_val};

fn main() {
    assert_eq!(
        // 16
        size_of::<&str>(),
        // pointer         + length
        size_of::<usize>() + size_of::<usize>()
    );
    assert_eq!(
        // 8
        size_of::<&&str>(),
        // just a pointer
        size_of::<usize>()
    );
    assert_eq!(
        // 24
        size_of::<String>(),
        // pointer         + length             + capacity
        size_of::<usize>() + size_of::<usize>() + size_of::<usize>()
    );
    assert_eq!(
        // 8
        size_of::<&String>(),
        // just a pointer
        size_of::<usize>()
    );

    // size of underlying "str" is 3 bytes
    let string_slice: &str = "str";
    let string: String = "str".to_owned();

    // `size_of_val` gives the size of the value pointed-to

    assert_eq!(
        // &str -> "str"
        size_of_val(string_slice),
        // size of "str" is 3 bytes
        3
    );
    assert_eq!(
        // &&str -> &str
        size_of_val(&string_slice),
        // size of &str is 16 bytes as indicated above
        16
    );
    assert_eq!(
        // &String -> String
        size_of_val(&string),
        // size of String is 24 bytes as indicated above
        24
    );
}
