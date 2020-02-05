use crate::stdweb::unstable::TryInto;
use crate::webgl_rendering_context::{WebGLRenderingContext, WebGLShader, GLenum, WebGLProgram};

/// Creates a shader of the given type, loads the source and compiles it.
///
/// ## Arguments
///
/// * `gl` - Webgl render context, acquired from a canvas.
/// * `kind` - Shader type.
/// * `src` - Shader source code.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context
pub fn load(gl: &WebGLRenderingContext, kind: GLenum, src: &str) -> Result<WebGLShader, String> {
    let shader = gl.create_shader(kind).unwrap();
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    let compile_status: bool = gl
        .get_shader_parameter(&shader, WebGLRenderingContext::COMPILE_STATUS)
        .try_into()
        .unwrap();

    if compile_status {
        Ok(shader)
    } else {
        let info = gl.get_shader_info_log(&shader).unwrap();
        gl.delete_shader(Some(&shader));
        Err(format!("An error occurred compiling the shaders: {}", info))
    }
}

/// Initialize a shader program, so that webgl understands how to draw
/// our data.
///
/// ## Arguments
///
/// * `gl` - Webgl render context, acquired from a canvas.
/// * `vsrc` - Vertex shader source code.
/// * `fsrc` - Fragment shader source code.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context
pub fn init(gl: &WebGLRenderingContext, vsrc: &str, fsrc: &str) -> Result<WebGLProgram, String> {
    let vertex_shader = load(gl, WebGLRenderingContext::VERTEX_SHADER, vsrc).unwrap();
    let fragment_shader = load(gl, WebGLRenderingContext::FRAGMENT_SHADER, fsrc).unwrap();

    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    let link_status: bool = gl
        .get_program_parameter(&program, WebGLRenderingContext::LINK_STATUS)
        .try_into()
        .unwrap();

    if link_status {
        Ok(program)
    } else {
        let info = gl.get_program_info_log(&program).unwrap();
        Err(format!("Unable to initialize the shader program: {}", info))
    }
}