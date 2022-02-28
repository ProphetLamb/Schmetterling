use gloo_console::warn;
use schling_common::{
    key::{KeyRegistrar, Shortcut},
    source::{Line, Text, ToDom},
};
use yew::prelude::*;
use yew_agent::Dispatched;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentUsing {
    Spaces,
    Tabs,
}

impl Default for IndentUsing {
    fn default() -> Self {
        IndentUsing::Tabs
    }
}

impl IndentUsing {
    fn char(self) -> char {
        match self {
            IndentUsing::Spaces => ' ',
            IndentUsing::Tabs => '\t',
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeStyle {
    indent_width: u32,
    indent_kind: IndentUsing,
    bracket_pairs: Vec<(char, char)>,
    tailing_line_break: bool,
}

pub enum SourceChanged {
    Change(Text),
    Insert(Text),
    Complete(Text),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: Text,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<(Event, SourceChanged)>,
}

#[function_component(CodeArea)]
pub fn code_area(props: &Props) -> Html {
    let Props {
        value,
        id,
        class,
        placeholder,
        onchange,
    } = props.clone();

    use_effect(move || {
        let mut hotkeys = KeyRegistrar::dispatcher();
        warn!("reg hotkeys");
        hotkeys.send(Shortcut::new_always(
            "Tab",
            Callback::from(|e: KeyboardEvent| {
                indent(e.clone());
                e.prevent_default();
                e.set_cancel_bubble(true);
            }),
        ));
        hotkeys.send(Shortcut::new_always(
            "Shift Tab",
            Callback::from(|e: KeyboardEvent| {
                outdent(e.clone());
                e.prevent_default();
                e.set_cancel_bubble(true);
            }),
        ));
        || {}
    });

    html! {
        <table {id} {class}>
            <tbody>
                {for value.symbols().map(render_line)}
            </tbody>
        </table>
    }
}

fn render_line(line: Line) -> Html {
    html! {
        <tr>
            <td id={line.gutter_id()} class="no-user-select" unselectable="on">{line.number()}</td>
            <td id={line.code_id()}>{line.to_dom()}</td>
        </tr>
    }
}

fn indent(e: KeyboardEvent) {
    warn!("indent")
}

fn outdent(e: KeyboardEvent) {
    warn!("outdent")
}
