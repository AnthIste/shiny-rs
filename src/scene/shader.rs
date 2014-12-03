extern crate gfx;

#[vertex_format]
pub struct Vertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..2],

    #[name = "a_Color"]
    pub color: [f32, ..3],
}

#[shader_param(CubeBatch)]
pub struct Params {
    #[name = "u_Transform"]
    pub transform: [[f32, ..4], ..4],
}

pub static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120

    attribute vec2 a_Pos;
    attribute vec3 a_Color;
    varying vec4 v_Color;
    uniform mat4 u_Transform;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        // gl_Position = vec4(a_Pos, 0.0, 1.0);
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core

    in vec2 a_Pos;
    in vec3 a_Color;
    out vec4 v_Color;
    uniform mat4 u_Transform;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        // gl_Position = vec4(a_Pos, 0.0, 1.0);
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
};

pub static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120

    varying vec4 v_Color;

    void main() {
        gl_FragColor = v_Color;
    }
"
GLSL_150: b"
    #version 150 core

    in vec4 v_Color;
    out vec4 o_Color;

    void main() {
        o_Color = v_Color;
    }
"
};