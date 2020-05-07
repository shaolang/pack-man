use regex::Regex;

fn main() {
    println!("Hello, World!");
}


fn pack(input: &Vec<&str>) -> Vec<String> {
    let mut s = String::new();
    let ignore_re = Regex::new(r"^\[.*\].+$|^\[/.+$").unwrap();
    let close_bracket_re = Regex::new(r"^[^\[]+].+$").unwrap();
    let mut start_matching = false;
    let mut result: Vec<String> = Vec::new();

    for v in input {
        if !start_matching {
            if v.starts_with("%%Page: ") {
                start_matching = true;
            }
            result.push(v.to_string());
        } else if ignore_re.is_match(v) {
            result.push(v.to_string());
        } else if v.starts_with('(') || (v.starts_with('[') && s.is_empty()) {
            s.clear();
            s.push_str(v);
        } else if !s.is_empty() && close_bracket_re.is_match(v) {
            s.push(' ');
            s.push_str(v);
            result.push(s.replace(r"\ ", ""));
            s.clear();
        } else if !s.is_empty() {
            s.push(' ');
            s.push_str(v);
        } else {
            result.push(v.to_string());
        }
    }

    if !s.is_empty() {
        result.push(s);
    }

    return result;
}


#[test]
fn pack_leaves_stuff_before_first_page_intact() {
    let input  = vec!("prelude", "[leave", "them", "intact]", "%%Page: x x",
                     "[0", "1] Td");
    let output = vec!("prelude", "[leave", "them", "intact]", "%%Page: x x",
                      "[0 1] Td");

    assert_eq!(pack(&input), output);
}

#[test]
fn pack_joins_double_digit_numbers_in_brackets() {
    let input  = vec!("whatever", "%%Page: y y",
                      "(a postscript string)", "[12", "11] TJ");
    let output = vec!("whatever", "%%Page: y y",
                      "(a postscript string) [12 11] TJ");

    assert_eq!(pack(&input), output);
}

#[test]
fn pack_joins_triple_digit_numbers_in_brackets() {
    let input  = vec!("%%Page: x x", "%%EndPageSetup", "[] 0 d",
                      "1 i", "/DeviceGray {} cs", "[123", "234] Td]");
    let output = vec!("%%Page: x x", "%%EndPageSetup", "[] 0 d",
                      "1 i", "/DeviceGray {} cs", "[123 234] Td]");

    assert_eq!(pack(&input), output);
}

#[test]
fn pack_leaves_stuff_intact_when_first_item_in_open_bracket_is_non_numeric() {
    let input  = vec!("%%Page: x x", "[/Indexed <", " 000", ">] something");

    assert_eq!(pack(&input), input);
}

#[test]
fn pack_joins_strings_in_parentheses() {
    let input  = vec!("%%Page: xyz xyz", r"(a post\", r"script \", "string)",
                      "[0", "1] TJ");
    let output = vec!("%%Page: xyz xyz", r"(a postscript string) [0 1] TJ");

    assert_eq!(pack(&input), output);
}

#[test]
fn pack_leaves_things_intact_when_there_are_no_brackets_or_parentheses() {
    let input = vec!("%%Page: x x", "0 0 cm");

    assert_eq!(pack(&input), input);
}
