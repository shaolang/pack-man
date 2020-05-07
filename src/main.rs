use std::env;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;

fn main() {
    let (iname, oname) = determine_io_filenames();
    let s = read_from_file(&iname);
    let input: Vec<&str> = s.split_terminator('\n').collect();

    let packed = pack(&input).join("\n");

    write_to_file(&oname, &packed);
}

// helper functions
///////////////////

fn determine_io_filenames() -> (String, String) {
    let mut args = env::args();
    let iname = &args.next().unwrap();
    let oname = if let Some(v) = args.next() {
        v
    } else {
        format!("packed-{}", iname.clone())
    };

    (iname.to_string(), oname.to_string())
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


fn read_from_file(fname: &str) -> String {
    let mut fin = File::open(fname)
        .expect(&format!("Unable to open {}", fname));
    let mut content = String::new();
    fin.read_to_string(&mut content).unwrap();

    return content;
}


fn write_to_file(fname: &str, content: &str) {
    let mut fout = File::create(fname)
        .expect(&format!("Unable to create {}", fname));
    write!(&mut fout, "{}", content)
        .expect(&format!("Unable to write to {}", fname));
}

// tests
////////

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
