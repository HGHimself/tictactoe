use regex::Regex;

macro_rules! lazy_regex {
    // `()` indicates that the macro takes no argument.
    ($pattern:literal, $name:ident) => {
        // The macro will expand into the contents of this block.
        lazy_static! {
            static ref $name: Regex = Regex::new($pattern).unwrap();
        }
    };
}

fn captures_len(input: &str, re: &Regex) -> i32 {
    let mut text = &input[..];
    let mut count = 0;

    while let Some(i) = re.find(text) {
        count += 1;
        text = &text[i.end()..];
    }

    return count;
}

fn o_4_in_row(input: &str) -> i32 {
    lazy_regex!(r"OOOO", RE);
    captures_len(input, &*RE)
}

// this will count EXXXExxxE as one since the middle E gets consumed
// method below will count the second sequence
fn o_2_empty_3_in_row_single(input: &str) -> i32 {
    lazy_regex!(r"EOOOE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn o_2_empty_3_in_row_double(input: &str) -> i32 {
    lazy_regex!(r"EOOOEOOOE", RE);
    captures_len(input, &*RE)
}

fn o_2_empty_3_in_row(input: &str) -> i32 {
    o_2_empty_3_in_row_single(input) + o_2_empty_3_in_row_double(input)
}

fn o_1_empty_3_in_row_left(input: &str) -> i32 {
    lazy_regex!(r"EOOO[^EO]", RE);
    captures_len(input, &*RE)
}

fn o_1_empty_3_in_row_right(input: &str) -> i32 {
    lazy_regex!(r"[^EO]OOOE", RE);
    captures_len(input, &*RE)
}

fn o_1_empty_3_in_row(input: &str) -> i32 {
    o_1_empty_3_in_row_right(input) + o_1_empty_3_in_row_left(input)
}

fn o_2_empty_2_in_row_single(input: &str) -> i32 {
    lazy_regex!(r"EOOE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn o_2_empty_2_in_row_double(input: &str) -> i32 {
    lazy_regex!(r"EOOEOOE", RE);
    captures_len(input, &*RE)
}

fn o_2_empty_2_in_row(input: &str) -> i32 {
    o_2_empty_2_in_row_single(input) + o_2_empty_2_in_row_double(input)
}

fn o_1_empty_2_in_row_left(input: &str) -> i32 {
    lazy_regex!(r"EOO[^EO]", RE);
    captures_len(input, &*RE)
}

fn o_1_empty_2_in_row_right(input: &str) -> i32 {
    lazy_regex!(r"[^EO]OOE", RE);
    captures_len(input, &*RE)
}

fn o_1_empty_2_in_row(input: &str) -> i32 {
    o_1_empty_2_in_row_right(input) + o_1_empty_2_in_row_left(input)
}

fn x_4_in_row(input: &str) -> i32 {
    lazy_regex!(r"XXXX", RE);
    captures_len(input, &*RE)
}

// this will count ExxxExxxE as one since the middle E gets consumed
// method below will count the second sequence
fn x_2_empty_3_in_row_single(input: &str) -> i32 {
    lazy_regex!(r"EXXXE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn x_2_empty_3_in_row_double(input: &str) -> i32 {
    lazy_regex!(r"EXXXEXXXE", RE);
    captures_len(input, &*RE)
}

fn x_2_empty_3_in_row(input: &str) -> i32 {
    x_2_empty_3_in_row_single(input) + x_2_empty_3_in_row_double(input)
}

fn x_1_empty_3_in_row_left(input: &str) -> i32 {
    lazy_regex!(r"EXXX[^EX]", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_3_in_row_right(input: &str) -> i32 {
    lazy_regex!(r"[^EX]XXXE", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_3_in_row(input: &str) -> i32 {
    x_1_empty_3_in_row_right(input) + x_1_empty_3_in_row_left(input)
}

fn x_2_empty_2_in_row_single(input: &str) -> i32 {
    lazy_regex!(r"EXXE", RE);
    captures_len(input, &*RE)
}

// this is funky because of a special case ExxxExxxE
// so we will count the second ExxxE here
fn x_2_empty_2_in_row_double(input: &str) -> i32 {
    lazy_regex!(r"EXXEXXE", RE);
    captures_len(input, &*RE)
}

fn x_2_empty_2_in_row(input: &str) -> i32 {
    x_2_empty_2_in_row_single(input) + x_2_empty_2_in_row_double(input)
}

fn x_1_empty_2_in_row_left(input: &str) -> i32 {
    lazy_regex!(r"EXX[^EX]", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_2_in_row_right(input: &str) -> i32 {
    lazy_regex!(r"[^EX]XXE", RE);
    captures_len(input, &*RE)
}

fn x_1_empty_2_in_row(input: &str) -> i32 {
    x_1_empty_2_in_row_right(input) + x_1_empty_2_in_row_left(input)
}

pub fn heuristic_counter_x(input: &str) -> i32 {
    let mut count: i32 = 0;
    if (x_4_in_row(input) != 0) {
        return 9999
    }
    if (o_4_in_row(input) != 0) {
        return -9999
    }

    count += 5 * x_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for me]
    count -= 10 * o_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for opponent]
    count += 3 * x_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for me]
    count -= 6 * o_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for opponent]
    count += (x_2_empty_2_in_row(input) + x_1_empty_2_in_row(input)); //[# of open-2-in-a-row for me]
    count -= (o_2_empty_2_in_row(input) + o_1_empty_2_in_row(input)); //[# of open-2-in-a-row for opponent]
    count
}

pub fn heuristic_counter_o(input: &str) -> i32 {
    let mut count: i32 = 0;
    if (o_4_in_row(input) != 0) {
        return 9999
    }
    if (x_4_in_row(input) != 0) {
        return -9999
    }

    count += 5 * o_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for me]
    count -= 10 * x_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for opponent]
    count += 3 * o_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for me]
    count -= 6 * x_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for opponent]
    count += (o_2_empty_2_in_row(input) + o_1_empty_2_in_row(input)); //[# of open-2-in-a-row for me]
    count -= (x_2_empty_2_in_row(input) + x_1_empty_2_in_row(input)); //[# of open-2-in-a-row for opponent]

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_x_2_empty_3_in_row() {
        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXX_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEEXXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_double("_EXXXEXXXE_"), 1);

        assert_eq!(x_2_empty_3_in_row_single("_EXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EEXXXEE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXE_"), 0);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEXXXE_"), 1);
        assert_eq!(x_2_empty_3_in_row_single("_EXXXEEXXXE_"), 2);

        assert_eq!(x_2_empty_3_in_row("_EXXXEXXXE_"), 2);
        assert_eq!(x_2_empty_3_in_row("_EXXXEEXXXE_"), 2);

    }

    #[test]
    fn test_x_1_empty_3_in_row() {
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXEE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_EXXXO_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXE_"), 1);

        assert_eq!(x_1_empty_3_in_row("_EXXXEXXX_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXXOXXXE_"), 2);
        assert_eq!(x_1_empty_3_in_row("_OXXXE_"), 1);
        assert_eq!(x_1_empty_3_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_3_in_row("_XXXEXXXEXX_"), 1);

    }


    #[test]
    fn test_x_2_empty_2_in_row() {
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);

        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EEXXEE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_2_empty_2_in_row("_EXXXEXXE_"), 1);
        assert_eq!(x_2_empty_2_in_row("_EXXEXXE_"), 2);
        assert_eq!(x_2_empty_2_in_row("_EXXEEXXE_"), 2);

    }

    #[test]
    fn test_x_1_empty_2_in_row() {
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);

        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);
        assert_eq!(x_1_empty_2_in_row("_EXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXO_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXX_"), 0);

        assert_eq!(x_1_empty_2_in_row("_XXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EEXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_OXXE_"), 1);
        assert_eq!(x_1_empty_2_in_row("_EXXE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EEXXEE_"), 0);
        assert_eq!(x_1_empty_2_in_row("_EXXEXX_"), 1);
        assert_eq!(x_1_empty_2_in_row("_XXEEXX_"), 2);

    }

    #[test]
    fn test_heuristic() {
        let input = "_EEXXEE_";
        assert_eq!(heuristic_counter_x(input), 1);
        assert_eq!(x_2_empty_2_in_row(input), 1);
        assert_eq!(x_1_empty_2_in_row(input), 0);

        let mut count = 0;
        count += 5 * x_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for me]
        assert_eq!(count, 0);
        count -= 10 * o_2_empty_3_in_row(input); //[# of two-side-open-3-in-a-row for opponent]
        assert_eq!(count, 0);
        count += 3 * x_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for me]
        assert_eq!(count, 0);
        count -= 6 * o_1_empty_3_in_row(input); //[# of one-side-open-3-in-a-row for opponent]
        assert_eq!(count, 0);
        count += (x_2_empty_2_in_row(input) + x_1_empty_2_in_row(input)); //[# of open-2-in-a-row for me]
        assert_eq!(count, 1);
        count -= (o_2_empty_2_in_row(input) + o_1_empty_2_in_row(input)); //[# of open-2-in-a-row for opponent]
        assert_eq!(count, 1);
    }
}
