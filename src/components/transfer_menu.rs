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
impl TryFrom<(u8,u8,u8,u8)> for RegionDisplay {
    type Error =  &'static str;

    fn try_from(value: (u8,u8,u8,u8)) -> Result<Self, Self::Error> {
        // This can only possibly fail if one of the coordinates is zero...
        Err("ni")
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
fn num_to_letters(num: u8) -> Option<String> {
    if num == 0 { return None } // Otherwise, we will not return none!
    // As another note, we can't represent higher than "IV" anyway;
    // thus there's no reason for a loop (26^n with n>1 will NOT occur).
    let mut text = "".to_string();
    let mut digit1 = num.div_euclid(26u8);
    let mut digit2 = num.rem_euclid(26u8);
    if digit1 > 0 && digit2 == 0u8 {
        digit1 -= 1;
        digit2 = 26;
    }
    if digit1 != 0 {
        text.push((64+digit1) as char)
    }
    text.push((64+digit2) as char);

    return Some(text.to_string());
}

#[cfg(test)]
mod tests {
    use super::{letters_to_num, num_to_letters, RegionDisplay};

    #[test]
    fn test_letters_to_num() {
        assert_eq!(letters_to_num("D"), Some(4));
        assert_eq!(letters_to_num("d"), None);
        assert_eq!(letters_to_num("AD"), Some(26+4));
        assert_eq!(letters_to_num("CG"), Some(3*26+7));
    }

    #[test]
    fn test_num_to_letters() {
        println!("27 is {:?}", num_to_letters(27));
        assert_eq!(num_to_letters(1), Some("A".to_string()));
        assert_eq!(num_to_letters(26), Some("Z".to_string()));
        assert_eq!(num_to_letters(27), Some("AA".to_string()));
        assert_eq!(num_to_letters(111), Some("DG".to_string()));
    }

    #[test]
    fn test_l2n_and_n2l() {
        assert_eq!(num_to_letters(letters_to_num("A").unwrap()), Some("A".to_string()));
        assert_eq!(num_to_letters(letters_to_num("BJ").unwrap()), Some("BJ".to_string()));
        for i in 1..=255 {
            assert_eq!(letters_to_num(&num_to_letters(i as u8).unwrap()), Some(i));
        }
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
