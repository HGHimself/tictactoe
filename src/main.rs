use regex::Regex;
#[macro_use] extern crate lazy_static;

macro_rules! lazy_regex {
    // `()` indicates that the macro takes no argument.
    ($pattern:literal, $name:ident) => {
        // The macro will expand into the contents of this block.
        lazy_static! {
            static ref $name: Regex = Regex::new($pattern).unwrap();
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

fn captures_len(input: &str, re: &Regex) -> usize {
    let mut text = &input[..];
    let mut count = 0;

    while let Some(i) = re.find(text) {
        count += 1;
        text = &text[i.end()..];
    }

    return count;
}

fn x_4_in_row(input: &str) -> usize {
    lazy_regex!(r"xxxx", RE);
    captures_len(input, &*RE)
}

// this will count ExxxExxxE as one since the middle E gets consumed
// method below will count the second sequence
fn x_2_empty_3_in_row_single(input: &str) -> usize {
    lazy_regex!(r"ExxxE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn x_2_empty_3_in_row_double(input: &str) -> usize {
    lazy_regex!(r"ExxxExxxE", RE);
    captures_len(input, &*RE)
}

fn x_2_empty_3_in_row(input: &str) -> usize {
    x_2_empty_3_in_row_single(input) + x_2_empty_3_in_row_double(input)
}

fn x_1_empty_3_in_row_left(input: &str) -> usize {
    lazy_regex!(r"Exxx[^Ex]", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_3_in_row_right(input: &str) -> usize {
    lazy_regex!(r"[^Ex]xxxE", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_3_in_row(input: &str) -> usize {
    x_1_empty_3_in_row_right(input) + x_1_empty_3_in_row_left(input)
}

fn x_2_empty_2_in_row_single(input: &str) -> usize {
    lazy_regex!(r"ExxE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn x_2_empty_2_in_row_double(input: &str) -> usize {
    lazy_regex!(r"ExxExxE", RE);
    captures_len(input, &*RE)
}

fn x_2_empty_2_in_row(input: &str) -> usize {
    x_2_empty_2_in_row_single(input) + x_2_empty_2_in_row_double(input)
}

fn x_1_empty_2_in_row_left(input: &str) -> usize {
    lazy_regex!(r"Exx[^Ex]", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_2_in_row_right(input: &str) -> usize {
    lazy_regex!(r"[^Ex]xxE", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_2_in_row(input: &str) -> usize {
    x_1_empty_2_in_row_right(input) + x_1_empty_2_in_row_left(input)
}

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
        assert_eq!(x_2_empty_3_in_row_double("_ExxxE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_ExxxExxx_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_ExxxEExxxE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_ExxxExxxE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_ExxxE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EExxxEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_ExxE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_ExxxExxxE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_ExxxEExxxE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_ExxxExxxE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_ExxxEExxxE_"), 2);
    }

    #[test]
    fn test_x_1_empty_3_in_row() {
        assert_eq!(x_1_empty_3_in_row("_Exxx_"), 1);
        assert_eq!(x_1_empty_3_in_row("_ExxxEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_Exxxo_"), 1);
        assert_eq!(x_1_empty_3_in_row("_ExxxE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_xxxE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_ExxxExxx_"), 1);
        assert_eq!(x_1_empty_3_in_row("_ExxxoxxxE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_oxxxE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_ExxE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_xxxExxxExx_"), 1);
    }


    #[test]
    fn test_x_2_empty_2_in_row() {
        assert_eq!(x_2_empty_2_in_row("_ExxE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EExxEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_ExxxE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_ExxxExxE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_ExxExxE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_ExxEExxE_"), 2);
    }

    #[test]
    fn test_x_1_empty_2_in_row() {
        assert_eq!(x_1_empty_2_in_row("_Exx_"), 1);
        assert_eq!(x_1_empty_2_in_row("_ExxEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_Exxo_"), 1);
        assert_eq!(x_1_empty_2_in_row("_ExxxE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_Exxx_"), 0);

        assert_eq!(x_1_empty_2_in_row("_xxE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EExxE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_oxxE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_ExxE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_ExxExx_"), 1);
        assert_eq!(x_1_empty_2_in_row("_xxEExx_"), 2);
    }
    //
    // #[test]
    // fn test_regex_captures() {
    //     let re = Regex::new(r"(Exx(E))").unwrap();
    //     assert_eq!(re.captures_len(), 2);
    // }
}
