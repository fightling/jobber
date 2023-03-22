mod add;
mod back;
mod delete;
mod edit;
mod export;
mod range;
mod start;

use regex::Regex;

// remove all ANSI color codes from output
fn clean(output: &Vec<u8>) -> String {
    let re = Regex::new(r"\u{1b}\[([0-9]{1,2}(;[0-9]{1,2})*)?[m|K]").unwrap();
    re.replace_all(std::str::from_utf8(output.as_slice()).unwrap(), "")
        .to_string()
}
