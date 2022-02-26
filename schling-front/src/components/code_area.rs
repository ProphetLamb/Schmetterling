use std::ops::Range;

use closure::closure;
use schling_common::key::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

const INDENT: &str = "  ";
fn outdent(target: &HtmlTextAreaElement) {
    fn outdent_line(
        text: &mut String,
        tracked: &mut Range<usize>,
        start: usize,
        end: usize,
    ) -> usize {
        let leading_ws = text[start..end]
            .find(|c: char| !c.is_whitespace() || c == '\n')
            .unwrap_or(end - start);
        // remove at max `indent` from the leading ws
        let outdent = leading_ws.min(INDENT.len());
        tracked.end -= outdent;
        //tracked.start -= outdent;

        let end = start + outdent;
        text.replace_range(start..end, "");
        outdent
    }

    let mut text = target.value();
    let selection = Selection::from_text_area(target);
    match selection {
        Selection::Area(start, end) => {
            // outdent line*s, track changes to selection
            let mut tracked = Range { start, end };
            // expand selection to fully encompass line*s
            let start = line_start(&text, start);
            let mut end = line_end(&text, end) - 2;

            let lstart = line_start(&text, end);
            tracked.start -= outdent_line(&mut text, &mut tracked, lstart, end);

            while let Some(lend) = &text[start..end].rfind('\n') {
                let lend = start + lend;
                let lstart = line_start(&text, lend);
                outdent_line(&mut text, &mut tracked, lstart, lend);
                end = lstart;
            }

            target.set_value(&text);
            target.set_selection_range(tracked.start as u32, tracked.end as u32);
        }
        Selection::Cursor(cursor) => {
            // outdent line* track changes to selection
            let mut tracked = Range {
                start: cursor,
                end: cursor,
            };
            // expand selection to fully encompass line*s
            let start = line_start(&text, cursor);
            let end = line_end(&text, cursor);
            outdent_line(&mut text, &mut tracked, start, end);

            target.set_value(&text);
            target.set_selection_range(tracked.start as u32, tracked.end as u32);
        }
        Selection::None => {}
    }
}

fn indent(target: &HtmlTextAreaElement) {
    fn indent_line(text: &mut String, tracked: &mut Range<usize>, end: usize) -> usize {
        let lstart = line_start(text, end);
        text.insert_str(lstart, INDENT);
        tracked.end += INDENT.len();
        lstart
    }

    let mut text = target.value();
    let selection = Selection::from_text_area(target);
    match selection {
        Selection::Area(start, end) => {
            if text[start..end].contains('\n') {
                // multiline -> indent selection, track changes to selection
                let mut tracked = Range { start, end };
                // expand selection to fully encompass line*s
                let start = line_start(&text, start);
                let mut end = line_end(&text, end);
                indent_line(&mut text, &mut tracked, end);
                tracked.start += INDENT.len();
                while let Some(lend) = &text[start..end].rfind('\n') {
                    let lend = start + lend;
                    let lstart = indent_line(&mut text, &mut tracked, lend);
                    end = lstart;
                }
                target.set_value(&text);
                target.set_selection_range(tracked.start as u32, tracked.end as u32);
            } else {
                // inline block -> replace selection
                let selection = Range { start, end };
                text.replace_range(selection, INDENT);
                target.set_value(&text);
            }
        }
        Selection::Cursor(cursor) => {
            text.insert_str(cursor, INDENT);
            target.set_value(&text);
        }
        Selection::None => {}
    }
}

#[derive(Debug, Clone, Copy)]
enum Selection {
    Area(usize, usize),
    Cursor(usize),
    None,
}

impl Default for Selection {
    fn default() -> Self {
        Selection::None
    }
}

impl Selection {
    fn from_text_area(target: &HtmlTextAreaElement) -> Self {
        // ff any selection returns Err the control is disabled or hidden.
        if let (Ok(start), Ok(end)) = (target.selection_start(), target.selection_end()) {
            match (start, end) {
                (Some(start), Some(end)) => {
                    if start == end {
                        Self::Cursor(start as usize)
                    } else {
                        Self::Area(start as usize, end as usize)
                    }
                }
                (Some(cursor), None) => Selection::Cursor(cursor as usize),
                _ => Self::default(),
            }
        } else {
            Self::default()
        }
    }
}

fn line_start(text: &str, start: usize) -> usize {
    let text = &text[..start];
    text.rfind('\n').map_or(0, |end| end + 1)
}

fn line_end(text: &str, end: usize) -> usize {
    let text = &text[end..];
    end + text.find('\n').unwrap_or(0)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(CodeArea)]
pub fn code_area(props: &Props) -> Html {
    let Props {
        id,
        class,
        value,
        placeholder,
        onchange,
    } = props.clone();

    let keys = use_mut_ref(KeyStack::default);
    let hotkeys = use_state_eq(register_hotkeys);

    let onkeydown = Callback::from(closure!(clone keys, |e: KeyboardEvent| {
        let mut keys = keys.borrow_mut();
        keys.key_down(&e.key());
        hotkeys.handle_keypress(&*keys, e);
    }));
    let onkeyup = Callback::from(closure!(clone keys, |e: KeyboardEvent| {
        let mut keys = keys.borrow_mut();
        keys.key_up(&e.key());
    }));
    html! {
        <textarea {id} {class} {placeholder} {value} {onchange} {onkeydown} {onkeyup} />
    }
}

fn register_hotkeys() -> KeyShortcuts {
    let mut hotkeys = KeyShortcuts::default();
    hotkeys.register_target("Tab", |target: HtmlTextAreaElement, e: KeyboardEvent| {
        indent(&target);
        e.prevent_default();
        e.set_cancel_bubble(true);
    });
    hotkeys.register_target(
        "Shift Tab",
        |target: HtmlTextAreaElement, e: KeyboardEvent| {
            outdent(&target);
            e.prevent_default();
            e.set_cancel_bubble(true);
        },
    );
    hotkeys
}
