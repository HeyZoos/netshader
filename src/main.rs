use serde::{Deserialize, Serialize};
use yew::agent::Bridged;
use yew::services::ConsoleService;
use yew::worker::{Agent, AgentLink, Bridge, Context, HandlerId};
use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
    worker: Box<dyn Bridge<Worker>>,
    console: ConsoleService,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg {
            value: "Message from worker".to_string(),
        });

        let worker = Worker::bridge(callback);

        Self {
            link,
            worker,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.console.log("[Model] Received message");

        self.worker.send(Request {
            value: "Message from model".to_string(),
        });

        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg {
            value: "foo".to_string(),
        });

        html! {
            <div>
                <button onclick=onclick>{ "Hello" }</button>
                <Variable name="foo" class="vec3" value="foo" />
            </div>
        }
    }
}

#[derive(Clone, Properties)]
struct Props {
    pub name: String,
    pub class: String,
    pub value: String,
}

struct Msg {
    value: String,
}

struct Variable {
    name: String,
    class: String,
    value: String,
    link: ComponentLink<Self>,
    _worker: Box<dyn Bridge<Worker>>,
}

impl Component for Variable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg {
            value: "Hello?".to_string(),
        });

        let worker = Worker::bridge(callback);
        Self {
            name: props.name,
            class: props.class,
            value: props.value,
            _worker: worker,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.value = msg.value;
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

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    value: String,
}

struct Worker {
    console: ConsoleService,
    _link: AgentLink<Self>,
}

impl Agent for Worker {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let console = ConsoleService::new();
        Self {
            console,
            _link: link,
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, _: Self::Input, _: HandlerId) {
        self.console.log("[Context Worker] Received input!");
    }
}

fn main() {
    yew::start_app::<Model>();
}
