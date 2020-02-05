extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate stdweb_derive;

mod webgl_rendering_context;

use serde::{Deserialize, Serialize};
use stdweb::js;
use stdweb::web::html_element::CanvasElement;
use webgl_rendering_context::WebGLRenderingContext;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

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
    canvas: NodeRef,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { canvas: Default::default() }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        let canvas = self.canvas.cast::<CanvasElement>().unwrap();
        let _gl: WebGLRenderingContext = canvas.get_context().unwrap();
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
                            <canvas ref=self.canvas.clone() class="h-100 w-100"></canvas>
                        </div>
                    </div>
                </div>
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut ace = AceService::new();
        ace.edit(&props.name);

        Self {
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

fn main() {
    yew::start_app::<Model>();
}
