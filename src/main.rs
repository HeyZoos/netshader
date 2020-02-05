extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate stdweb_derive;

mod models;
mod shader;
mod webgl_rendering_context;

use serde::{Deserialize, Serialize};
use stdweb::js;
use stdweb::web::html_element::CanvasElement;
use webgl_rendering_context::WebGLRenderingContext;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};
use crate::webgl_rendering_context::{WebGLProgram, WebGLUniformLocation};
use crate::models::{Model, Cube};

use std::f32::consts::PI;

pub const DEFAULT_VERTEX: &str = r#"
    attribute vec4 aVertexPosition;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    void main() {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
    }
"#;

pub const DEFAULT_FRAGMENT: &str = r#"precision mediump float;
varying vec3 vColor;
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

struct State {
    canvas_ref: NodeRef,
    canvas: Option<CanvasElement>,
    gl: Option<WebGLRenderingContext>,
    shader_program: Option<WebGLProgram>,
    vertex_position: Option<u32>,
    projection_matrix: Option<WebGLUniformLocation>,
    model_view_matrix: Option<WebGLUniformLocation>,
    model: Box<dyn Model>,
}

impl State {
    fn render(&self) {
        let gl = self.gl.as_ref().unwrap();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.enable(WebGLRenderingContext::DEPTH_TEST);
        gl.depth_func(WebGLRenderingContext::LEQUAL);

        // Clear the canvas before we start drawing on it.
        gl.clear(WebGLRenderingContext::COLOR_BUFFER_BIT | WebGLRenderingContext::DEPTH_BUFFER_BIT);

        // Create a perspective matrix, a special matrix that is
        // used to simulate the distortion of perspective in a camera.
        // Our field of view is 45 degrees, with a width/height
        // ratio that matches the display size of the canvas
        // and we only want to see objects between 0.1 units
        // and 100 units away from the camera.

        let canvas_height = gl.canvas().height() as f32;
        let canvas_width = gl.canvas().width() as f32;

        let fov = 45f32 * PI / 180f32;   // in radians
        let aspect = canvas_width / canvas_height;
        let z_near = 0.1;
        let z_far = 100.0;

        // note: glmatrix.js always has the first argument
        // as the destination to receive the result.

        let projection_matrix = nalgebra_glm::perspective::<f32>(aspect, fov, z_near, z_far);

        // Set the drawing position to the "identity" point, which is
        // the center of the scene.
        let mut model_view_matrix = nalgebra_glm::mat4(
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        );

        // Now move the drawing position a bit to where we want to
        // start drawing the square.

        let translation = nalgebra_glm::vec3(0.0, 0.0, -6.0);
        model_view_matrix = nalgebra_glm::translate(&model_view_matrix, &translation);

        // Tell WebGL how to pull out the positions from the position
        // buffer into the vertexPosition attribute.
        {
            let num_components = 2;  // pull out 2 values per iteration
            let normalize = false;   // don't normalize
            let stride = 0;          // how many bytes to get from one set of values to the next

            // 0 = use type and numComponents above
            let offset = 0;          // how many bytes inside the buffer to start from

//            gl.bind_buffer(WebGLRenderingContext::ARRAY_BUFFER, self.model.positions().as_ref());

            gl.vertex_attrib_pointer(
                self.vertex_position.unwrap(),
                num_components,
                WebGLRenderingContext::FLOAT,
                normalize,
                stride,
                offset
            );

            gl.enable_vertex_attrib_array(self.vertex_position.unwrap());
        }

        // Tell WebGL to use our program when drawing

        gl.use_program(self.shader_program.as_ref());

        // Set the shader uniforms

//        gl.uniformMatrix4fv(
//            programInfo.uniformLocations.projectionMatrix,
//            false,
//            projectionMatrix);
//        gl.uniformMatrix4fv(
//            programInfo.uniformLocations.modelViewMatrix,
//            false,
//            modelViewMatrix);
//
//        {
//            const offset = 0;
//            const vertexCount = 4;
//            gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
//        }
    }
}

impl Component for State {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            canvas_ref: Default::default(),
            canvas: None,
            gl: None,
            shader_program: None,
            vertex_position: None,
            model_view_matrix: None,
            projection_matrix: None,
            model: Box::new(Cube::new())
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.canvas = Some(self.canvas_ref.cast::<CanvasElement>().unwrap());
        self.gl = Some(self.canvas.as_ref().unwrap().get_context().unwrap());
        self.gl.as_ref().unwrap().clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.as_ref().unwrap().clear(WebGLRenderingContext::COLOR_BUFFER_BIT);

        self.shader_program = Some(shader::init(self.gl.as_ref().unwrap(), DEFAULT_VERTEX, DEFAULT_FRAGMENT).unwrap());

        self.projection_matrix = Some(self.gl
            .as_ref()
            .unwrap()
            .get_uniform_location(self.shader_program.as_ref().unwrap(), "uProjectionMatrix")
            .unwrap());

        self.model_view_matrix = Some(self.gl
            .as_ref()
            .unwrap()
            .get_uniform_location(self.shader_program.as_ref().unwrap(), "uModelViewMatrix")
            .unwrap());

        self.render();

        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
                            <canvas ref=self.canvas_ref.clone() class="h-100 w-100"></canvas>
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
    yew::start_app::<State>();
}
