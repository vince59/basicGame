/* Structure FireSet (gestion des feux) */

use crate::Shape;
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct FiresSet {
    pub fires: Vec<Shape>,
    fire_sprite: AnimatedSprite,
    fire_texture: Texture2D,
}

impl FiresSet {
    pub async fn new() -> FiresSet {
        let mut fire_sprite = AnimatedSprite::new(
            64,
            64,
            &[Animation {
                name: "fire".to_string(),
                row: 0,
                frames: 10,
                fps: 12,
            }],
            true,
        );
       fire_sprite.set_animation(0);
        let fire_texture: Texture2D = load_texture("fire.png")
            .await
            .expect("Couldn't load file");
        fire_texture.set_filter(FilterMode::Nearest);

        FiresSet {
            fires: vec![],
            fire_texture,
            fire_sprite
        }
    }

    pub fn display(&self) {
        for fire in &self.fires {
            let frame = self.fire_sprite.frame();

            draw_texture_ex(
                &self.fire_texture,
                fire.x - fire.size / 2.0,
                fire.y - fire.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(fire.size, fire.size)),
                    source: Some(frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    // suppression des feux
    pub fn clear(&mut self) {
        self.fires.clear();
    }

    // Ajout d'un feux
    pub fn push(&mut self, shape: &Shape) {
        self.fires.push(shape.clone());
    }

    pub fn get_list(&mut self) -> &mut Vec<Shape> {
        &mut self.fires
    }

    pub fn update(&mut self, delta_time: f32) {
        self.fire_sprite.set_animation(0);
        self.display();
        self.fire_sprite.update();
    }
}
