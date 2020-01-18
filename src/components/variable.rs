use yew::html::InputData;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Variable {
    name: String,
    class: String,
    value: String,
    link: ComponentLink<Self>,
    console: ConsoleService,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub name: String,
    pub class: String,
    pub value: String,
}

pub struct Msg {
    value: String,
}

impl Component for Variable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            name: props.name,
            class: props.class,
            value: props.value,
            console: ConsoleService::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.value = msg.value;
        self.console.log(&self.value);
        true
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(|input_data: InputData| Msg {
            value: input_data.value,
        });

        html! {
            <div>
                <p>{ &self.name }</p>
                <p>{ &self.class }</p>
                <input oninput=oninput>{ &self.value }</input>
            </div>
        }
    }
}
