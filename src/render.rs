use miniquad::{graphics::*, Context};
use specs_blit::PixelBuffer;
use std::slice;

const VERTEX: &str = r#"#version 100
attribute vec2 pos;
attribute vec2 uv;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(pos, 0, 1);
    texcoord = uv;
}
"#;

const FRAGMENT: &str = r#"#version 100
varying lowp vec2 texcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}"#;

const META: ShaderMeta = ShaderMeta {
    images: &["tex"],
    uniforms: UniformBlockLayout { uniforms: &[] },
};

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

/// A wrapper around the OpenGL calls so the main file won't be polluted.
pub struct Render {
    pipeline: Pipeline,
    texture: Texture,
    bindings: Bindings,
}

impl Render {
    /// Setup the OpenGL pipeline and the texture for the framebuffer.
    pub fn new(ctx: &mut Context, width: usize, height: usize) -> Self {
        // Setup the quad vertices
        let vertices: [Vertex; 4] = [
            Vertex {
                pos: Vec2 { x: -1.0, y: -1.0 },
                uv: Vec2 { x: 0.0, y: 1.0 },
            },
            Vertex {
                pos: Vec2 { x: 1.0, y: -1.0 },
                uv: Vec2 { x: 1.0, y: 1.0 },
            },
            Vertex {
                pos: Vec2 { x: 1.0, y: 1.0 },
                uv: Vec2 { x: 1.0, y: 0.0 },
            },
            Vertex {
                pos: Vec2 { x: -1.0, y: 1.0 },
                uv: Vec2 { x: 0.0, y: 0.0 },
            },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        // Setup the quad indices
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        // Create an OpenGL texture for our framebuffer
        let texture = Texture::new_render_texture(
            ctx,
            TextureParams {
                format: TextureFormat::RGBA8,
                // Use nearest filtering because we want to maintain pixels without blur
                filter: FilterMode::Nearest,
                width: width as u32,
                height: height as u32,
                ..Default::default()
            },
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        // Create an OpenGL pipeline
        let shader = Shader::new(ctx, VERTEX, FRAGMENT, META);
        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Self {
            pipeline,
            bindings,
            texture,
        }
    }

    /// Render the pixel buffer.
    pub fn render(&mut self, ctx: &mut Context, buffer: &PixelBuffer) {
        // Convert the [u32] buffer to [u8]
        // Unsafe because the safe way to do this didn't have the same performance
        let bytes = unsafe {
            slice::from_raw_parts(
                buffer.pixels().as_ptr() as *const u8,
                buffer.pixels().len() * 4,
            )
        };

        // Update the texture
        self.texture.update(ctx, &bytes);

        // Render the texture quad
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);

        ctx.apply_bindings(&self.bindings);

        // Draw the 6 indices with 1 instance
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}
