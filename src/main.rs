use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod components;

use components::Variable;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Variable name="foo" class="vec3" value="foo" />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
