#![allow(non_snake_case)]

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::data::{transfer::Transfer, transfer_region::Region};

use super::states::{CurrentTransfer, MainState};

#[function_component]
pub fn TransferMenu() -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();

    let on_name_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                ct_dispatch.reduce_mut(|state| {
                    state.transfer.name = input.value().clone();
                });
            }
        })
    };

    let on_src_region_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(rd) = RegionDisplay::try_from(input.value()) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.transfer_region.source_region = Region::from(&rd);
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
                        state.transfer.transfer_region.dest_region = Region::from(&rd);
                    });
                    input.set_custom_validity("");
                } else {
                    input.set_custom_validity("Invalid region.")
                }
            }
        })
    };

    let on_source_interleave_x_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(num) = input.value().parse::<i8>() {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.transfer_region.interleave_source =
                            (num, state.transfer.transfer_region.interleave_source.1);
                    });
                }
            }
        })
    };
    let on_source_interleave_y_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if let Ok(num) = input.value().parse::<i8>() {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.transfer_region.interleave_source =
                            (state.transfer.transfer_region.interleave_source.0, num);
                    });
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
                        state.transfer.transfer_region.interleave_dest =
                            (num, state.transfer.transfer_region.interleave_dest.1);
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
                        state.transfer.transfer_region.interleave_dest =
                            (state.transfer.transfer_region.interleave_dest.0, num);
                    });
                }
            }
        })
    };

    let on_volume_change = {
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: Event| {
            let input = e
                .target()
                .expect("Event must have target")
                .dyn_into::<HtmlInputElement>()
                .ok()
                .expect("Must have been emitted by input");
            if let Ok(num) = input.value().parse::<f32>() {
                ct_dispatch.reduce_mut(|state| {
                    state.transfer.volume = num;
                });
            }
        })
    };

    let new_transfer_button_callback = {
        let main_dispatch = main_dispatch.clone();
        let main_state = main_state.clone();
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |_: MouseEvent| {
            main_dispatch.reduce_mut(|state| {
                state.selected_transfer = Uuid::nil();
            });
            ct_dispatch.reduce_mut(|state| {
                state.transfer = Transfer::default();
                state.transfer.source_id = main_state.selected_source_plate;
                state.transfer.dest_id = main_state.selected_dest_plate;
            });
        })
    };

    let save_transfer_button_callback = {
        let main_dispatch = main_dispatch.clone();
        let main_state = main_state.clone();
        let ct_state = ct_state.clone();

        Callback::from(move |_: MouseEvent| {
            log::debug!("Button pressed");
            if main_state.selected_transfer.is_nil() {
                if let Some(spi) = main_state
                    .source_plates
                    .iter()
                    .find(|spi| spi.get_uuid() == main_state.selected_source_plate)
                {
                    if let Some(dpi) = main_state
                        .destination_plates
                        .iter()
                        .find(|dpi| dpi.get_uuid() == main_state.selected_dest_plate)
                    {
                        let new_transfer = Transfer::new(
                            spi.clone(),
                            dpi.clone(),
                            ct_state.transfer.transfer_region,
                            ct_state.transfer.name.clone(),
                        );
                        main_dispatch.reduce_mut(|state| {
                            state.transfers.push(new_transfer);
                            state.selected_transfer = state
                                .transfers
                                .last()
                                .expect("An element should have just been added")
                                .get_uuid();
                        });
                    }
                }
            } else {
                if let Some(index) = main_state
                    .transfers
                    .iter()
                    .position(|t| t.get_uuid() == main_state.selected_transfer)
                {
                    main_dispatch.reduce_mut(|state| {
                        state.transfers[index] = ct_state.transfer.clone();
                    });
                }
            }
        })
    };

    let delete_transfer_button_callback = {
        let main_dispatch = main_dispatch.clone();
        let main_state = main_state.clone();
        let ct_state = ct_state.clone();
        let new_callback = new_transfer_button_callback.clone();

        Callback::from(move |e: MouseEvent| {
            if main_state.selected_transfer.is_nil() {
                () // Maybe reset transfer?
            } else {
                if let Some(index) = main_state
                    .transfers
                    .iter()
                    .position(|t| t.get_uuid() == ct_state.transfer.get_uuid())
                {
                    main_dispatch.reduce_mut(|state| {
                        state.transfers.remove(index);
                        state.selected_transfer = Uuid::nil();
                    });
                    new_callback.emit(e); // We need a new transfer now
                }
            }
        })
    };

    html! {
        <div class="transfer_menu">
            <form>
            <div>
                <label for="name"><h3>{"Name:"}</h3></label>
                <input type="text" name="name"
                onchange={on_name_change}
                value={ct_state.transfer.name.clone()}/>
            </div>
            <div>
                <label for="src_region"><h3>{"Source Region:"}</h3></label>
                <input type="text" name="src_region"
                onchange={on_src_region_change}
                value={RegionDisplay::from(&ct_state.transfer.transfer_region.source_region).text}/>
            </div>
            <div>
                <label for="dest_region"><h3>{"Destination Region:"}</h3></label>
                <input type="text" name="dest_region"
                onchange={on_dest_region_change}
                value={RegionDisplay::from(&ct_state.transfer.transfer_region.dest_region).text}/>
            </div>
            <div>
            <h3>{"Source Interleave "}</h3>
            <label for="source_interleave_x">{"Row:"}</label>
            <input type="number" name="source_interleave_x"
            onchange={on_source_interleave_x_change}
            value={ct_state.transfer.transfer_region.interleave_source.0.to_string()}/>
            <label for="source_interleave_y">{"Col:"}</label>
            <input type="number" name="source_interleave_y"
            onchange={on_source_interleave_y_change}
            value={ct_state.transfer.transfer_region.interleave_source.1.to_string()}/>
            </div>
            <div>
            <h3>{"Destination Interleave "}</h3>
            <label for="dest_interleave_x">{"Row:"}</label>
            <input type="number" name="dest_interleave_x"
            onchange={on_dest_interleave_x_change}
            value={ct_state.transfer.transfer_region.interleave_dest.0.to_string()}/>
            <label for="dest_interleave_y">{"Col:"}</label>
            <input type="number" name="dest_interleave_y"
            onchange={on_dest_interleave_y_change}
            value={ct_state.transfer.transfer_region.interleave_dest.1.to_string()}/>
            </div>
            <div>
            <label for="volume"><h3>{"Volume"}</h3></label>
            <input type="number" name="volume" class="volume_input"
            min="0" step="0.1"
            onchange={on_volume_change}
            value={ct_state.transfer.volume.to_string()}/>
            </div>
            <div id="controls">
            <input type="button" name="new_transfer" onclick={new_transfer_button_callback}
            value={"New"} />
            <input type="button" name="save_transfer" onclick={save_transfer_button_callback}
            value={"Save"} />
            <input type="button" name="delete_transfer" onclick={delete_transfer_button_callback}
            value={"Delete"} />
            </div>
            </form>
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
            if captures.len() != 5 {
                return Err("Not enough capture groups");
            }
            let col_start = letters_to_num(&captures[1]).ok_or("Column start failed to parse")?;
            let col_end = letters_to_num(&captures[3]).ok_or("Column end failed to parse")?;
            let row_start: u8 = captures[2]
                .parse::<u8>()
                .or(Err("Row start failed to parse"))?;
            let row_end: u8 = captures[4]
                .parse::<u8>()
                .or(Err("Row end failed to parse"))?;
            return Ok(RegionDisplay {
                text: value,
                col_start,
                row_start,
                col_end,
                row_end,
            });
        } else {
            return Err("Regex match failed");
        }
    }
}
impl From<&Region> for RegionDisplay {
    fn from(value: &Region) -> Self {
        match *value {
            Region::Point((col, row)) => {
                RegionDisplay::try_from((col, row, col, row)).ok().unwrap()
            }
            Region::Rect(c1, c2) => RegionDisplay::try_from((c1.0, c1.1, c2.0, c2.1))
                .ok()
                .unwrap(),
        }
    }
}
impl From<&RegionDisplay> for Region {
    fn from(value: &RegionDisplay) -> Self {
        if value.col_start == value.col_end && value.row_start == value.row_end {
            Region::Point((value.col_start, value.row_start))
        } else {
            Region::Rect(
                (value.col_start, value.row_start),
                (value.col_end, value.row_end),
            )
        }
    }
}
impl TryFrom<(u8, u8, u8, u8)> for RegionDisplay {
    type Error = &'static str;

    fn try_from(value: (u8, u8, u8, u8)) -> Result<Self, Self::Error> {
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
        if n < 65 || n > 90 {
            return None;
        }
        num = num.checked_add((26_i32.pow(i as u32) * (n as i32 - 64)).try_into().ok()?)?;
    }
    return Some(num);
}
pub fn num_to_letters(num: u8) -> Option<String> {
    if num == 0 {
        return None;
    } // Otherwise, we will not return none!
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
        text.push((64 + digit1) as char)
    }
    text.push((64 + digit2) as char);

    return Some(text.to_string());
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::{letters_to_num, num_to_letters, RegionDisplay};

    #[test]
    #[wasm_bindgen_test]
    fn test_letters_to_num() {
        assert_eq!(letters_to_num("D"), Some(4));
        assert_eq!(letters_to_num("d"), None);
        assert_eq!(letters_to_num("AD"), Some(26 + 4));
        assert_eq!(letters_to_num("CG"), Some(3 * 26 + 7));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_num_to_letters() {
        println!("27 is {:?}", num_to_letters(27));
        assert_eq!(num_to_letters(1), Some("A".to_string()));
        assert_eq!(num_to_letters(26), Some("Z".to_string()));
        assert_eq!(num_to_letters(27), Some("AA".to_string()));
        assert_eq!(num_to_letters(111), Some("DG".to_string()));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_l2n_and_n2l() {
        assert_eq!(
            num_to_letters(letters_to_num("A").unwrap()),
            Some("A".to_string())
        );
        assert_eq!(
            num_to_letters(letters_to_num("BJ").unwrap()),
            Some("BJ".to_string())
        );
        for i in 1..=255 {
            assert_eq!(letters_to_num(&num_to_letters(i as u8).unwrap()), Some(i));
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_try_from_string_for_regiondisplay() {
        let desired = RegionDisplay {
            text: "A1:E5".to_string(),
            row_start: 1,
            row_end: 5,
            col_start: 1,
            col_end: 5,
        };
        assert_eq!(desired, "A1:E5".to_string().try_into().unwrap());
    }
}
