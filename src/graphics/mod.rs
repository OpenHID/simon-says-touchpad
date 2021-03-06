/*!
A quad based renderer used for raymarching techniques. 

> You can render entire worlds with only 2 triangles! ~ Inigo Quilez, Shadertoy

```no_run
mod graphics;

use graphics::Event;

fn main() {
    let renderer = graphics::Renderer::new();

    // Fragment Shader
    renderer.fs = "\
// Uniforms for time, mouse position, resolution, 
// etc are prepended by the renderer.

void main() {
    outColor = vec4(1.);
}
";
    loop {
        match renderer.update() {
            Event::Close => return
            _ => ()
        }
    }
}
```
*/

extern crate glium;

use self::glium::{DisplayBuild, Display, VertexBuffer, IndexBuffer, Program};
use self::glium::index::PrimitiveType;
pub use self::glium::glutin::Event;
use self::glium::Surface;

use std::time::{Instant};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    auv: [f32; 2],
}

implement_vertex!(Vertex, position, auv);

static VBO: [Vertex; 4] = [Vertex {
                               position: [1.0, -1.0],
                               auv: [1.0, 0.0],
                           },
                           Vertex {
                               position: [-1.0, -1.0],
                               auv: [0.0, 0.0],
                           },
                           Vertex {
                               position: [1.0, 1.0],
                               auv: [1.0, 1.0],
                           },
                           Vertex {
                               position: [-1.0, 1.0],
                               auv: [0.0, 1.0],
                           }];

static IBO: [u16; 6] = [0, 1, 2, 1, 2, 3];

pub struct Renderer {
    pub display: Display,
    program: Program,
    vbo: VertexBuffer<Vertex>,
    ibo: IndexBuffer<u16>,
    mouse: [f32; 4],
    resolution: [f32; 2],
    now: Instant
}


impl Renderer {
    pub fn new(fs: &str) -> Renderer {

        let display = glium::glutin::WindowBuilder::new()
            .with_fullscreen(glium::glutin::get_primary_monitor())
            .build_glium()
            .unwrap();

        let vbo = glium::VertexBuffer::new(&display, &VBO).unwrap();

        let ibo = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &IBO).unwrap();

        let mut frag = String::new();

        frag.push_str(include_str!{concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/shaders/frag.glsl")});
        frag.push_str(fs);

        let program = program!(&display, 
            140 =>  {
                    vertex: include_str!{concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/shaders/vert.glsl")},

                    fragment: frag.as_str()

                }
            110 => {
                    vertex: include_str!{concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/shaders/vert.glsl")},

                    fragment: frag.as_str()

                }
            100 => {
                    vertex: include_str!{concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/shaders/vert.glsl")},

                    fragment: frag.as_str()
                    }
            ).unwrap();

        let r = display.get_framebuffer_dimensions();
        
        Renderer {
            display,
            program,
            vbo,
            ibo,
            mouse: [0.; 4],
            resolution: [r.0 as f32, r.1 as f32],
            now: Instant::now()
        }
    }
    
    pub fn update<F>(&mut self, other_uniforms: [f32; 4], mut lambda: F) where F: FnMut(&Event) {

        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 0.0);

        let uniforms = uniform! {
            mouse: self.mouse,
            resolution: self.resolution,
            time: self.now.elapsed().as_secs() as f32 + (self.now.elapsed().subsec_nanos() as f32 / 1000000000.0),
            other: other_uniforms
        };

        target
            .draw(&self.vbo,
                  &self.ibo,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .unwrap();

        target.finish().unwrap();

        // Poll IO
        {
            let events = self.display.poll_events();
            
            for event in events {
                
                lambda(&event);

                match event {

                    Event::MouseInput(glium::glutin::ElementState::Pressed,
                                    glium::glutin::MouseButton::Left) => {
                        self.mouse = [self.mouse[0], self.mouse[1], 1., self.mouse[3]];
                    }

                    Event::MouseInput(glium::glutin::ElementState::Released,
                                    glium::glutin::MouseButton::Left) => {
                        self.mouse = [self.mouse[0], self.mouse[1], 0., self.mouse[3]];
                    }

                    Event::MouseMoved(x, y) => {
                        self.mouse = [x as f32, y as f32, self.mouse[2], self.mouse[3]];
                    }
                    _ => (),
                }
            }
        }
    }
}
