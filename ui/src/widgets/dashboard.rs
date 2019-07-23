use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::dashboard as frame;
use yew::{html, Properties, ShouldRender};

pub type DashboardWidget = WidgetModel<Model>;

pub struct Model {
    dashboard: Option<frame::Dashboard>,
    selected_page: usize,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub dashboard: Option<frame::Dashboard>,
}

#[derive(Debug)]
pub enum Msg {
    SelectPage(usize),
}

impl Widget for Model {
    type Message = Msg;
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            dashboard: None,
            selected_page: 0,
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.dashboard = props.dashboard.to_owned();
        None
    }

    fn handle_inner(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPage(idx) => {
                self.selected_page = idx;
                true
            }
        }
    }

    fn main_view(&self) -> View<Self> {
        if let Some(dashboard) = self.dashboard.as_ref() {
            let page = dashboard.pages.get(self.selected_page).cloned();
            html! {
                <div class="dashboard",>
                    <div class="sidebar",>
                        <div class="header",>
                            <p class="title",>{ &dashboard.title }</p>
                        </div>
                        <ul class="menu",>
                            { for dashboard.pages.iter().enumerate().map(|(idx, page)| self.view_page_title(idx, page)) }
                        </ul>
                    </div>
                    <div class="content",>
                        <widgets::Page: page=page, />
                    </div>
                </div>
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}

impl Model {
    fn view_page_title(&self, idx: usize, page: &frame::Page) -> View<Self> {
        let class = if self.selected_page == idx { "item selected" } else { "item" };
        html! {
            <li class=class, onclick=|_| Msg::SelectPage(idx).into(),>{ &page.title }</li>
        }
    }
}
