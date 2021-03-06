use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type SpinnerWidget = WidgetModel<Model>;

pub struct Model {}

impl Widget for Model {
    type Message = ();
    type Properties = ();

    fn produce(_: &Self::Properties) -> Self {
        Self {}
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <div class="spinner",>
                <img src="./spinner.svg", width=200, />
            </div>
        }
    }
}
