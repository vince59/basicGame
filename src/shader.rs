use macroquad::prelude::*;

const VERTEX_SHADER: &str = include_str!("vertex_shader.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment_shader.glsl");

pub struct Shader {
    texture: Texture2D,
    material: Material,
}

impl Shader {
    pub fn new() -> Shader {
        let img = Image::gen_image_color(1, 1, WHITE);
        let texture = Texture2D::from_image(&img);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("screen_size", UniformType::Float2),
                ],
                ..Default::default()
            },
        )
        .unwrap();
        Shader {
            texture,
            material,
        }
    }

    pub fn display(&mut self) {
        gl_use_material(&self.material);
        self.material.set_uniform("time", get_time() as f32);
        self.material.set_uniform("screen_size", vec2(screen_width(), screen_height()));
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
