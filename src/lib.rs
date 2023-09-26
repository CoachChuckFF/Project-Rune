extern  crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod programs;
mod common;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameEngineClient {
    gl: GL,
    program_color_2d: programs::color_2ds::Color2D,
    tick: u64,
}

#[wasm_bindgen]
impl GameEngineClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            program_color_2d: programs::color_2ds::Color2D::new(&gl),
            gl,
            tick: 0,
        }   
    }

    pub fn start(&mut self, _time: f32, _width: f32, _height: f32) -> Result<(), JsValue> {
        log("Starting Game");
        Ok(())
    }

    pub fn update(&mut self, _time: f32, _width: f32, _height: f32) -> Result<(), JsValue> {

        self.tick += 1;

        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.program_color_2d.render(
            &self.gl, 
            0., // bottom
            10., // top
            0., // left
            10., // right
            10., // canvas_width
            10., // canvas_height
            self.tick,
        );
    }
}