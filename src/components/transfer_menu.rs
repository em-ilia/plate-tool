#![allow(non_snake_case)]
use dioxus::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;

#[inline_props]
pub fn TransferMenu(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "transfer_menu",
            form{
                label {
                    r#for: "src_region",
                    "Source Region:"
                },
                input {
                    r#type: "text",
                    name: "src_region",
                },
                label {
                    r#for: "dest_region",
                    "Destination Region:"
                },
                input {
                    r#type: "text",
                    name: "dest_region",
                }
            }
        }
    })
}

#[derive(PartialEq, Eq, Debug)]
struct RegionDisplay {
    text: String,
    row_start: u8,
    row_end: u8,
    col_start: u8,
    col_end: u8
}

impl TryFrom<String> for RegionDisplay {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref REGION_REGEX: Regex = Regex::new(r"([A-Z]+)(\d+):([A-Z]+)(\d+)").unwrap();
        }
        if let Some(captures) = REGION_REGEX.captures(&value) {
            if captures.len() != 5 { return Err("Not enough capture groups") }
            let col_start = letters_to_num(&captures[1]).ok_or("Row start failed to parse")?;
            let col_end = letters_to_num(&captures[3]).ok_or("Row end failed to parse")?;
            let row_start: u8 = captures[2].parse::<u8>().or(Err("Col start failed to parse"))?;
            let row_end: u8 = captures[4].parse::<u8>().or(Err("Col end failed to parse"))?;
            return Ok(RegionDisplay { 
                text: value,
                row_start,
                row_end,
                col_start,
                col_end })
        } else {
            return Err("Regex match failed")
        }
    }

}
fn letters_to_num(letters: &str) -> Option<u8> {
    let mut num: u8 = 0;
    for (i, letter) in letters.chars().rev().enumerate() {
        let n = letter as u8;
        if n < 65 || n > 90 { return None }
        num = num.checked_add((26_i32.pow(i as u32)*(n as i32 - 64)).try_into().ok()?)?;
    }
    return Some(num)
}

#[cfg(test)]
mod tests {
    use super::{letters_to_num, RegionDisplay};

    #[test]
    fn test_letters_to_num() {
        assert_eq!(letters_to_num("D"), Some(4));
        assert_eq!(letters_to_num("d"), None);
        assert_eq!(letters_to_num("AD"), Some(26+4));
        assert_eq!(letters_to_num("CG"), Some(3*26+7));
    }

    #[test]
    fn test_try_from_string_for_regiondisplay() {
        let desired = RegionDisplay {
            text: "A1:E5".to_string(),
            row_start: 1,
            row_end: 5,
            col_start: 1,
            col_end: 5
        };
        assert_eq!(desired, "A1:E5".to_string().try_into().unwrap());
    }
}
