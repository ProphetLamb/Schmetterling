use web_sys::{FocusEvent, MouseEvent};
use yew::prelude::Event;

use crate::{components::edt::Presentation, id, markup::Markup};

pub enum Proj {
    Add(id::Doc),
}

pub enum Doc {
    Add(id::Card),
    CardTitle((id::Card, String, Event)),
    CardContent((id::Card, Markup, Event)),
    Title(String),
    TitleMode(Presentation),
}

pub enum Card {
    Mode(Presentation),
    DoubleClick(MouseEvent),
    Blur(FocusEvent),
}
