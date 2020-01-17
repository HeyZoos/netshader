use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model { }

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { }
    }
}

fn main() {
    yew::start_app::<Model>();
}
