#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use regex::Regex;
use lazy_static::lazy_static;

use crate::data::transfer_region::Region;

use super::states::{CurrentTransfer};

#[function_component]
pub fn TransferMenu() -> Html {
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();

    let on_src_region_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(rd) = RegionDisplay::try_from(input.value()) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.source_region = Region::from(&rd);
                    });
                    input.set_custom_validity("");
                } else {
                    input.set_custom_validity("Invalid region.")
                }
            }
        })
    };
    let on_dest_region_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(rd) = RegionDisplay::try_from(input.value()) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.dest_region = Region::from(&rd);
                    });
                    input.set_custom_validity("");
                } else {
                    input.set_custom_validity("Invalid region.")
                }
            }
        })
    };
    
    let on_dest_interleave_x_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(num) = input.value().parse::<i8>() {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.interleave_dest = (num, state.transfer.interleave_dest.1);
                    });
                }
            }
        })
    };
    let on_dest_interleave_y_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(num) = input.value().parse::<i8>() {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.interleave_dest = (state.transfer.interleave_dest.0, num);
                    });
                }
            }
        })
    };

    html! {
        <div class="transfer_menu">
            <form>
            <div>
                <label for="src_region">{"Source Region:"}</label>
                <input type="text" name="src_region"
                onchange={on_src_region_change}
                value={RegionDisplay::from(&ct_state.transfer.source_region).text}/>
            </div>
            <div>
                <label for="dest_region">{"Destination Region:"}</label>
                <input type="text" name="dest_region"
                onchange={on_dest_region_change}
                value={RegionDisplay::from(&ct_state.transfer.dest_region).text}/>
            </div>
            <div>
            {"Destination Interleave "}
            <label for="dest_interleave_x">{"X:"}</label>
            <input type="number" name="dest_interleave_x"
            onchange={on_dest_interleave_x_change} value={ct_state.transfer.interleave_dest.0.to_string()}/>
            <label for="dest_interleave_y">{"Y:"}</label>
            <input type="number" name="dest_interleave_y"
            onchange={on_dest_interleave_y_change} value={ct_state.transfer.interleave_dest.1.to_string()}/>
            </div>
            <input type="button" name="create_transfer" value={"Create"} />
            </form>
            <button>{"Refresh"}</button>
        </div>
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegionDisplay {
    pub text: String,
    pub col_start: u8,
    pub row_start: u8,
    pub col_end: u8,
    pub row_end: u8,
}

impl TryFrom<String> for RegionDisplay {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref REGION_REGEX: Regex = Regex::new(r"([A-Z]+)(\d+):([A-Z]+)(\d+)").unwrap();
        }
        if let Some(captures) = REGION_REGEX.captures(&value) {
            if captures.len() != 5 { return Err("Not enough capture groups") }
            let col_start = letters_to_num(&captures[1]).ok_or("Column start failed to parse")?;
            let col_end = letters_to_num(&captures[3]).ok_or("Column end failed to parse")?;
            let row_start: u8 = captures[2].parse::<u8>().or(Err("Row start failed to parse"))?;
            let row_end: u8 = captures[4].parse::<u8>().or(Err("Row end failed to parse"))?;
            return Ok(RegionDisplay { 
                text: value,
                col_start,
                row_start,
                col_end,
                row_end,
                })
        } else {
            return Err("Regex match failed")
        }
    }

}
impl From<&Region> for RegionDisplay {
    fn from(value: &Region) -> Self {
        match *value {
            Region::Point((col, row)) => RegionDisplay::try_from((col,row,col,row)).ok().unwrap(),
            Region::Rect(c1, c2) =>
                RegionDisplay::try_from((c1.0,c1.1,c2.0,c2.1)).ok().unwrap()
        }
    }
}
impl From<&RegionDisplay> for Region {
    fn from(value: &RegionDisplay) -> Self {
        if value.col_start == value.col_end && value.row_start == value.row_end {
            Region::Point((value.col_start, value.row_start))
        } else {
            Region::Rect((value.col_start, value.row_start), (value.col_end, value.row_end))
        }
    }
}
impl TryFrom<(u8,u8,u8,u8)> for RegionDisplay {
    type Error =  &'static str;

    fn try_from(value: (u8,u8,u8,u8)) -> Result<Self, Self::Error> {
        // (Column Start, Row Start, Column End, Row End)
        // This can only possibly fail if one of the coordinates is zero...
        let cs = num_to_letters(value.0).ok_or("Column start failed to parse")?;
        let ce = num_to_letters(value.2).ok_or("Column end failed to parse")?;
        Ok(RegionDisplay {
            text: format!("{}{}:{}{}", cs, value.1, ce, value.3),
            col_start: value.0,
            row_start: value.1,
            col_end: value.2,
            row_end: value.3,
        })
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
