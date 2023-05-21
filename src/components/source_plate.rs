#![allow(non_snake_case)]
use dioxus::prelude::*;

static STYLE: &'static str = include_str!("plate.css");

#[derive(PartialEq, Props)]
pub struct SourcePlateProps {
    width: u8,
    height: u8,
}
struct SelectionState {
    m_start: Option<(u8, u8)>,
    m_end: Option<(u8, u8)>,
    m_stat: bool,
}

pub fn SourcePlate(cx: Scope<SourcePlateProps>) -> Element {
    use_shared_state_provider(cx, || SelectionState {
        m_start: None,
        m_end: None,
        m_stat: false,
    });

    cx.render(rsx! {
        style {
            vec![STYLE].into_iter().map(|s| rsx!{s}) // This is stupid
        }
        SourcePlateSelectionIndicator {}
        SourcePlateSelectionController {}
        table {
            draggable: "false",
            for i in 1..=cx.props.height {
                tr {
                    draggable: "false",
                    for j in 1..=cx.props.width {
                        SourcePlateCell {i: i, j: j}
                    }
                }
            },
        }
    })
}

#[inline_props]
fn SourcePlateCell(cx: Scope<PlateCellProps>, i: u8, j: u8, color: Option<String>) -> Element {
    let selection_state = use_shared_state::<SelectionState>(cx).unwrap();
    let selected = in_rect(
        selection_state.read().m_start,
        selection_state.read().m_end,
        (*i, *j),
    );
    let selected_class = match selected {
        true => "current_select",
        false => "",
    };
    let color_string = match color {
        Some(c) => c.clone(),
        None => "None".to_string(),
    };

    cx.render(rsx! {
        td {
            class: "plate_cell {selected_class}",
            draggable: "false",
            style: "background: {color_string}",
            onmousedown: move |_| {
                selection_state.write().m_start = Some((*i,*j));
                selection_state.write().m_end = None;
                selection_state.write().m_stat = true;
            },
            onmouseover: move |_| {
                if selection_state.read().m_stat {
                    selection_state.write().m_end = Some((*i,*j))
                }
            },
            onmouseup: move |_| {
                selection_state.write().m_stat = false
            },
            div {
                class: "plate_cell_inner"
            }
        }
    })
}

fn in_rect(corner1: Option<(u8, u8)>, corner2: Option<(u8, u8)>, pt: (u8, u8)) -> bool {
    if let (Some(c1), Some(c2)) = (corner1, corner2) {
        return pt.0 <= u8::max(c1.0, c2.0)
            && pt.0 >= u8::min(c1.0, c2.0)
            && pt.1 <= u8::max(c1.1, c2.1)
            && pt.1 >= u8::min(c1.1, c2.1);
    } else {
        return false;
    }
}

// This is a dummy component for design purposes only
fn SourcePlateSelectionIndicator(cx: Scope) -> Element {
    let selection_state = use_shared_state::<SelectionState>(cx).unwrap();
    let start_str = match selection_state.read().m_start {
        Some(start) => format!("{},{}", start.0, start.1),
        None => "None".to_string(),
    };
    let end_str = match selection_state.read().m_end {
        Some(end) => format!("{},{}", end.0, end.1),
        None => "None".to_string(),
    };

    cx.render(rsx! {
        p { start_str ", and " end_str }
    })
}

fn SourcePlateSelectionController(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            button {
                "Select"
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::in_rect;

    // in_rect tests
    #[test]
    fn test_in_rect1() {
        // Test in center of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt = (5, 5);
        assert!(in_rect(Some(c1), Some(c2), pt));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt));
    }

    #[test]
    fn test_in_rect2() {
        // Test on top/bottom edges of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (1, 5);
        let pt2 = (10, 5);
        assert!(in_rect(Some(c1), Some(c2), pt1));
        assert!(in_rect(Some(c1), Some(c2), pt2));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt1));
        assert!(in_rect(Some(c2), Some(c1), pt2));
    }

    #[test]
    fn test_in_rect3() {
        // Test on left/right edges of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (5, 1);
        let pt2 = (5, 10);
        assert!(in_rect(Some(c1), Some(c2), pt1));
        assert!(in_rect(Some(c1), Some(c2), pt2));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt1));
        assert!(in_rect(Some(c2), Some(c1), pt2));
    }

    #[test]
    fn test_in_rect4() {
        // Test cases that should fail
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (0, 0);
        let pt2 = (15, 15);
        assert_eq!(false, in_rect(Some(c1), Some(c2), pt1));
        assert_eq!(false, in_rect(Some(c1), Some(c2), pt2));
    }
}
