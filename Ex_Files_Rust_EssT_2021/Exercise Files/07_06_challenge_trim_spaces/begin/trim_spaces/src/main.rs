fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */
fn trim_spaces(s: &str) -> &str {
    return trim_left(trim_right(s));
}

fn trim_left(s: &str) -> &str {
    let mut first_char_idx = 0;
    for (index, byte) in s.bytes().enumerate() {
        if byte != b' ' {
            break;
        }
        first_char_idx = index;
    }
    return &s[first_char_idx..]
}

fn trim_right(s: &str) -> &str {
    let mut last_char_idx = s.len();

    for (index, byte) in s.bytes().enumerate().rev() {
        last_char_idx = index;
        if byte != b' ' {
            break;
        }
        // if byte != b' ' {
        //     return &s[..index + 1]
        // }
    }
    return &s[..last_char_idx + 1]
}
