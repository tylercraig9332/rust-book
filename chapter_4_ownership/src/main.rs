use std::fmt::Write;

fn main() {
    let mut s = String::from("hello");
    let s_literal = "hello";
    s.push_str(", world!");
    // This will not complie since it will consume s and not let us print
    // consume_ownership(s);
    share_ownership(&mut s);
    println!("{s}");
    println!("--- slices ---");
    let s = String::from("hello world");
    let fw = first_word(&s);
    println!("{fw}");
}

fn consume_ownership(str: String) {
    println!("I now own {str}. muahhaha");
}

fn share_ownership(str: &mut String) {
    println!("I am just using {str}");
    str.push_str(" Foo Bar");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] //s
}
