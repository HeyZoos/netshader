use serde::{Deserialize, Serialize};
use stdweb::js;
use yew::agent::Bridged;
use yew::services::ConsoleService;
use yew::worker::{Agent, AgentLink, Bridge, Context, HandlerId};
use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

pub const DEFAULT_VERTEX: &str = r#"attribute vec3 position;
uniform mat4 Pmatrix;
uniform mat4 Vmatrix;
uniform mat4 Mmatrix;
attribute vec3 color;
varying vec3 vColor;
void main() {
    gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
    vColor = color;
}
"#;

pub const DEFAULT_FRAGMENT: &str = r#"precision mediump float;
varying vec3 vColor;
void main() {
    gl_FragColor = vec4(vColor, 1.);
}
"#;

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
        html! {
            <div class="container-fluid h-100">
                <div class="row h-100">
                    <div class="col-sm py-4 pl-4 pr-4">
                        <div class="h-50 pb-4">
                            <EditorComponent name="fragment-editor" class="border rounded h-100 w-100" />
                        </div>
                        <EditorComponent name="vertex-editor" class="border rounded h-50 w-100" />
                    </div>
                    <div class="col-sm py-4 pl-0 pr-4">
                        <div class="h-25 pb-4">
                            <div class="h-100 border rounded"></div>
                        </div>
                        <div class="h-75 border rounded">
                            <WebGlComponent />
                        </div>
                    </div>
                </div>
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

struct AceService;

impl AceService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn edit(&mut self, id: &str) {
        js! {
            Promise.resolve().then(() => {
                window.ace = ace;
                window.editor = window.ace.edit(@{id});
                window.editor.setTheme("ace/theme/tomorrow_night");
                window.editor.getSession().setMode("ace/mode/javascript");
                window.editor.setValue("Hello World");
                window.editor.clearSelection();
            });
        };
    }
}

struct EditorComponent {
    ace: AceService,
    link: ComponentLink<Self>,
    name: String,
    class: String,
}

#[derive(Clone, Properties)]
struct EditorComponentProperties {
    name: String,
    class: String,
}

impl Component for EditorComponent {
    type Message = ();
    type Properties = EditorComponentProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ace = AceService::new();

        ace.edit(&props.name);

        Self {
            ace,
            link,
            name: props.name,
            class: props.class,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id={ &self.name } class={ &self.class }></div>
        }
    }
}

struct WebGlService;

impl WebGlService {
    fn new() -> Self {
        js! {
            Promise.resolve().then(() => {
                window.canvas = document.getElementById("canvas");
                window.gl = window.canvas.getContext("webgl");
            });
        }

        Self {}
    }

    fn create_shader(&self, name: &str, source: &str, vertex: bool) {
        if vertex {
            js! {
                window[@{ name }] = gl.createShader(gl.VERTEX_SHADER);
            }
        } else {
            js! {
                window[@{ name }] = gl.createShader(gl.FRAGMENT_SHADER);
            }
        }

        js! {
            gl.shaderSource(window[@{ name }], @{ source });
            gl.compileShader(window[@{ name }]);
        }
    }

    fn create_program(&self, name: &str, shader_names: Vec<&str>) {
        js! {
            window[@{ name }] = gl.createProgram();
        }

        for shader_name in shader_names.iter() {
            js! {
                console.log(@{ shader_name });
                gl.attachShader(window[@{ name }], window[@{ shader_name }]);
            }
        }

        js! {
            gl.linkProgram(window[@{ name }]);
        }
    }
}

struct WebGlComponent {
    gl: WebGlService,
    link: ComponentLink<Self>,
}

struct WebGlComponentMsg {}

impl Component for WebGlComponent {
    type Message = WebGlComponentMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            gl: WebGlService::new(),
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.gl.create_shader("vertex", DEFAULT_VERTEX, true);
        self.gl.create_shader("fragment", DEFAULT_FRAGMENT, false);

        self.gl
            .create_program("program", vec!["vertex", "fragment"]);
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| WebGlComponentMsg {});
        html! {
            <>
                <canvas id="canvas" class="h-100 w-100"></canvas>
                <button onclick=onclick>{ "Update Canvas" }</button>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
