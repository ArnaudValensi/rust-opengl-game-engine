// extern crate glbs;
extern crate gl;
extern crate glutin;

use common::{Window, process_events};
use self::glutin::GlContext;

use self::gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub fn main_1_2_2() {
    // glfw: initialize and configure
    // ------------------------------
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT);

    let (shaderProgram, VAO) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(512);
        infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        }

        // fragment shader
        let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragmentShader);
        // check for shader compile errors
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        }

        // link shaders
        let shaderProgram = gl::CreateProgram();
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);
        // check for linking errors
        gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        }
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 12] = [
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0   // top left
        ];
        let indices = [ // note that we start from 0!
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // remember: do NOT unbind the EBO while a VAO is active as the bound element buffer object IS stored in the VAO; keep the EBO bound.
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shaderProgram, VAO)
    };

    // render loop
    // -----------
    while window.running {
        // events
        // -----
        process_events(&mut window);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::UseProgram(shaderProgram);
            gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            // gl::BindVertexArray(0); // no need to unbind it every time
        }

        window.gl_window.swap_buffers().unwrap();
    }
}