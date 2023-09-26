use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;

use crate::{common, shaders};

pub struct Color2D {
    program: WebGlProgram,
    rectangle_vertices_array_length: usize,
    rectangle_vertices_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = common::link_program(
            &gl,
            &shaders::vertex::color_2ds::SHADER,
            &shaders::fragment::color_2ds::SHADER,
        ).unwrap();

        let vertices_rectangle: [f32; 12] = [
            0., 1.,
            0., 0.,
            1., 1.,
            1., 1.,
            0., 0.,
            1., 0.,
        ];

        let vertices_rectangle_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_rectangle_location = vertices_rectangle.as_ptr() as u32 / 4;
        let vertices_array = js_sys::Float32Array::new(&vertices_rectangle_memory_buffer)
            .subarray(vertices_rectangle_location, vertices_rectangle_location + vertices_rectangle.len() as u32);

        let buffer_rectangle = gl.create_buffer().ok_or("failed to create buffer").unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rectangle));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "u_opacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
            rectangle_vertices_array_length: vertices_rectangle.len(),
            rectangle_vertices_buffer: buffer_rectangle,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_width: f32,
        canvas_height: f32,
        tick: u64,

    ) {
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rectangle_vertices_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        let red = (tick % 255) as f32 / 255.;
        let green = ((tick + 85) % 255) as f32 / 255.; // Offset by roughly 1/3 of 255
        let blue = ((tick + 170) % 255) as f32 / 255.; // Offset by roughly 2/3 of 255
        gl.uniform4f(Some(&self.u_color), red, green, blue, 1.0);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        let transformation_matrix = common::translation_matrix(
            left * 2. / canvas_width - 1.,
            bottom * 2. / canvas_height - 1.,
            0.,
        );

        let scale_matrix = common::scaling_matrix(
            (right - left) * 2. / canvas_width,
            (top - bottom) * 2. / canvas_height,
            1.,
        );

        let transform = common::multiply_matrix_4(transformation_matrix, scale_matrix);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.rectangle_vertices_array_length / 2) as i32);
    }
}