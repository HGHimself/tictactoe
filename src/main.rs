use regex::Regex;
#[macro_use] extern crate lazy_static;

macro_rules! lazy_regex {
    // `()` indicates that the macro takes no argument.
    ($pattern:literal) => {
        // The macro will expand into the contents of this block.
        lazy_static! {
            static ref RE: Regex = Regex::new($pattern).unwrap();
        }
    };
}

fn main() {
    println!("Hello, world!");
}

fn xy_to_linear(x: u32, y: u32, h: u32, w: u32) -> Option<u32> {
    if x > w - 1 || y > h - 1 {
        None
    } else {
        let i = (y * w) + x;
        Some(i)
    }
}

fn alphabet_to_index(char: &str) -> Option<usize> {
    if char == "E" {
        Some(0)
    } else if char == "/" {
        Some(1)
    } else if char == "x" {
        Some(2)
    } else if char == "o" {
        Some(3)
    } else {
        None
    }
}

fn captures_len(input: &str, find: &dyn Fn(&str) -> Option<regex::Match>) -> usize {
    let mut text = &input[..];
    let mut count = 0;

    while let Some(i) = find(text) {
        count += 1;
        text = &text[i.end()..];
    }

    return count;
}

// this will count ExxxExxxE as one since the middle E gets consumed
// method below will count the second sequence
fn x_2_empty_3_in_row_single(input: &str) -> usize {
    lazy_regex!(r"ExxxE");

    let mut text = &input[..];
    let mut count = 0;

    while let Some(i) = RE.find(text) {
        count += 1;
        text = &text[i.end()..];
    }

    return count;
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn x_2_empty_3_in_row_double(input: &str) -> usize {
    lazy_regex!(r"ExxxExxxE");

    let mut text = &input[..];
    let mut count = 0;

    while let Some(i) = RE.find(text) {
        count += 1;
        text = &text[i.end()..];
    }

    return count;
}

fn x_2_empty_3_in_row(input: &str) -> usize {
    x_2_empty_3_in_row_single(input) + x_2_empty_3_in_row_double(input)
}
//
// fn x_1_empty_3_in_row(input: &str) -> bool {
//     lazy_static! {
//         static ref RE1: Regex = Regex::new(r"[^E]xxxE").unwrap();
//         static ref RE2: Regex = Regex::new(r"Exxx[^E]").unwrap();
//     }
//     RE1.is_match(input) || RE2.is_match(input)
// }
//
// fn x_2_empty_2_in_row(input: &str) -> bool {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"ExxE").unwrap();
//     }
//     RE.is_match(input)
// }
//
// fn x_1_empty_2_in_row(input: &str) -> bool {
//     lazy_static! {
//         static ref RE1: Regex = Regex::new(r"Exx$").unwrap();
//         static ref RE2: Regex = Regex::new(r"Exx[^Ex]").unwrap();
//
//         static ref RE3: Regex = Regex::new(r"^xxE").unwrap();
//         static ref RE4: Regex = Regex::new(r"[^Ex]xxE").unwrap();
//     }
//     RE1.is_match(input) || RE2.is_match(input) || RE3.is_match(input) || RE4.is_match(input)
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xy_to_linear() {
        let w = 5;
        let h = 4;
        assert_eq!(xy_to_linear( 1, 1, h, w), Some(6));
        assert_eq!(xy_to_linear( 0, 0, h, w), Some(0));
        assert_eq!(xy_to_linear( 4, 0, h, w), Some(4));
        assert_eq!(xy_to_linear( 4, 2, h, w), Some(14));
        assert_eq!(xy_to_linear( 4, 4, h, w), None);
        assert_eq!(xy_to_linear( 5, 3, h, w), None);
        assert_eq!(xy_to_linear( 1, 2, h, w), Some(11));
    }

    #[test]
    fn test_x_2_empty_3_in_row() {

        assert_eq!(x_2_empty_3_in_row_double("ExxxE"), 0);
        assert_eq!(x_2_empty_3_in_row_double("ExxxExxx"), 0);
        assert_eq!(x_2_empty_3_in_row_double("ExxxEExxxE"), 0);
        assert_eq!(x_2_empty_3_in_row_double("ExxxExxxE"), 1);

        assert_eq!(x_2_empty_3_in_row_single("ExxxE"), 1);
        assert_eq!(x_2_empty_3_in_row_single("EExxxEE"), 1);
        assert_eq!(x_2_empty_3_in_row_single("ExxE"), 0);
        assert_eq!(x_2_empty_3_in_row_single("ExxxExxxE"), 1);
        assert_eq!(x_2_empty_3_in_row_single("ExxxEExxxE"), 2);

        assert_eq!(x_2_empty_3_in_row("ExxxExxxE"), 2);
        assert_eq!(x_2_empty_3_in_row("ExxxEExxxE"), 2);
    }
    //
    // #[test]
    // fn test_x_2_empty_2_in_row() {
    //     assert_eq!(x_2_empty_2_in_row("ExxE"), true);
    //     assert_eq!(x_2_empty_2_in_row("EExxEE"), true);
    //     assert_eq!(x_2_empty_2_in_row("ExxxE"), false);
    //     assert_eq!(x_2_empty_2_in_row("ExxxExxE"), true);
    // }
    //
    // #[test]
    // fn test_x_1_empty_2_in_row() {
    //     assert_eq!(x_1_empty_2_in_row("Exx"), true);
    //     assert_eq!(x_1_empty_2_in_row("ExxEE"), false);
    //     assert_eq!(x_1_empty_2_in_row("Exxo"), true);
    //     assert_eq!(x_1_empty_2_in_row("ExxxE"), false);
    //     assert_eq!(x_1_empty_2_in_row("Exxx"), false);
    //
    //     assert_eq!(x_1_empty_2_in_row("xxE"), true);
    //     assert_eq!(x_1_empty_2_in_row("EExxE"), false);
    //     assert_eq!(x_1_empty_2_in_row("oxxE"), true);
    //     assert_eq!(x_1_empty_2_in_row("ExxE"), false);
    //     assert_eq!(x_2_empty_2_in_row("ExxxExx"), true);
    // }
    //
    // #[test]
    // fn test_regex_captures() {
    //     let re = Regex::new(r"(Exx(E))").unwrap();
    //     assert_eq!(re.captures_len(), 2);
    // }
}
