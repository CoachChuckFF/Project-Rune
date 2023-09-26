use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn link_program(
    gl: &WebGlRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| String::from("Unable to create program object"))?;

    let vertex_shader = compile_shader(&gl, GL::VERTEX_SHADER, vertex_source).unwrap();
    let fragment_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, fragment_source).unwrap();

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_matrix = [0.; 16];

    return_matrix[0] = 1.;
    return_matrix[5] = 1.;
    return_matrix[10] = 1.;
    return_matrix[15] = 1.;

    return_matrix[12] = tx;
    return_matrix[13] = ty;
    return_matrix[14] = tz;

    return_matrix
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut return_matrix = [0.; 16];

    return_matrix[0] = sx;
    return_matrix[5] = sy;
    return_matrix[10] = sz;
    return_matrix[15] = 1.;

    return_matrix
}

pub fn multiply_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_matrix = [0.; 16];

    return_matrix[0] = a[0] * b[0] + a[4] * b[1] + a[8] * b[2] + a[12] * b[3];
    return_matrix[1] = a[1] * b[0] + a[5] * b[1] + a[9] * b[2] + a[13] * b[3];
    return_matrix[2] = a[2] * b[0] + a[6] * b[1] + a[10] * b[2] + a[14] * b[3];
    return_matrix[3] = a[3] * b[0] + a[7] * b[1] + a[11] * b[2] + a[15] * b[3];

    return_matrix[4] = a[0] * b[4] + a[4] * b[5] + a[8] * b[6] + a[12] * b[7];
    return_matrix[5] = a[1] * b[4] + a[5] * b[5] + a[9] * b[6] + a[13] * b[7];
    return_matrix[6] = a[2] * b[4] + a[6] * b[5] + a[10] * b[6] + a[14] * b[7];
    return_matrix[7] = a[3] * b[4] + a[7] * b[5] + a[11] * b[6] + a[15] * b[7];

    return_matrix[8] = a[0] * b[8] + a[4] * b[9] + a[8] * b[10] + a[12] * b[11];
    return_matrix[9] = a[1] * b[8] + a[5] * b[9] + a[9] * b[10] + a[13] * b[11];
    return_matrix[10] = a[2] * b[8] + a[6] * b[9] + a[10] * b[10] + a[14] * b[11];
    return_matrix[11] = a[3] * b[8] + a[7] * b[9] + a[11] * b[10] + a[15] * b[11];

    return_matrix[12] = a[0] * b[12] + a[4] * b[13] + a[8] * b[14] + a[12] * b[15];
    return_matrix[13] = a[1] * b[12] + a[5] * b[13] + a[9] * b[14] + a[13] * b[15];
    return_matrix[14] = a[2] * b[12] + a[6] * b[13] + a[10] * b[14] + a[14] * b[15];
    return_matrix[15] = a[3] * b[12] + a[7] * b[13] + a[11] * b[14] + a[15] * b[15];

    return_matrix
}