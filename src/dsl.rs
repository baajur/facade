#![allow(non_snake_case)]

use protocol::{self as frame, Id, Layout, Value, Widget};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: impl Into<Layout>) -> frame::Scene {
    frame::Scene::FullScreen(value.into())
}

pub fn Spinner() -> frame::Scene {
    frame::Scene::Spinner
}

pub fn Dashboard(
    title: impl Into<Value>,
    pages: impl IntoIterator<Item = frame::dashboard::Page>,
) -> frame::Scene {
    let dashboard = frame::dashboard::Dashboard {
        title: title.into(),
        pages: pages.into_iter().collect(),
    };
    frame::Scene::Dashboard(dashboard)
}

pub fn Page(
    title: impl Into<Value>,
    subtitle: impl Into<Value>,
    body: impl Into<Layout>,
) -> frame::dashboard::Page {
    frame::dashboard::Page {
        title: title.into(),
        subtitle: subtitle.into(),
        body: body.into(),
    }
}

pub fn Menu(value: impl IntoIterator<Item = frame::MenuItem>) -> frame::Menu {
    frame::Menu {
        items: value.into_iter().collect(),
    }
}

pub fn Item(value: impl Into<Value>) -> frame::MenuItem {
    frame::MenuItem {
        caption: value.into(),
    }
}

// ╔═╗┌─┐┌┐┌┌┬┐┌─┐┬┌┐┌┌─┐┬─┐┌─┐
// ║  │ ││││ │ ├─┤││││├┤ ├┬┘└─┐
// ╚═╝└─┘┘└┘ ┴ ┴ ┴┴┘└┘└─┘┴└─└─┘

pub fn Panel(value: impl Into<Layout>) -> Layout {
    let panel = frame::Panel {
        body: value.into(),
    };
    let container = frame::Container::Panel(panel);
    Layout::Container(Box::new(container))
}

// ╦  ┌─┐┬ ┬┌─┐┬ ┬┌┬┐
// ║  ├─┤└┬┘│ ││ │ │
// ╩═╝┴ ┴ ┴ └─┘└─┘ ┴

pub fn Blank() -> frame::Layout {
    frame::Layout::Blank
}

pub fn Row(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}

pub fn Column(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Column(layouts)
}

// ╦ ╦┬┌┬┐┌─┐┌─┐┌┬┐┌─┐
// ║║║│ │││ ┬├┤  │ └─┐
// ╚╩╝┴─┴┘└─┘└─┘ ┴ └─┘

pub fn Dynamic(value: impl Into<Id>) -> Layout {
    Layout::Widget(Widget::Dynamic(value.into()))
}

pub fn Fixed(value: impl Into<Value>) -> Layout {
    Layout::Widget(Widget::Fixed(value.into()))
}
