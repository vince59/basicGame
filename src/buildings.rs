/* Structure BuildingsSet (gestion des bâtiments) */

use crate::Shape;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::experimental::animation::Animation;
use macroquad::prelude::*;

pub struct BuildingsSet {
    pub buildings: Vec<Shape>,
    pub building_texture: Texture2D,
    pub building_sprite: AnimatedSprite,
}

impl BuildingsSet {
    pub async fn new() -> BuildingsSet {
        let building_texture: Texture2D = load_texture("temple.png")
            .await
            .expect("Couldn't load file");
        building_texture.set_filter(FilterMode::Nearest);
        let mut building_sprite = AnimatedSprite::new(
            49,
            58,
            &[Animation {
                name: "temple".to_string(),
                row: 0,
                frames: 1,
                fps: 12,
            }],
            true,
        );
        building_sprite.set_animation(0);
        BuildingsSet {
            buildings: vec![],
            building_texture,
            building_sprite,
        }
    }

    // suppression des balles et des explosions
    pub fn clear(&mut self) {
        self.buildings.clear();
    }

    // Ajout d'une balle
    pub fn push(&mut self, shape: Shape) {
        self.buildings.push(shape);
    }

    // affichage des balles et des explosions
    pub fn display(&mut self) {
        let building_frame = self.building_sprite.frame();
        for building in &self.buildings {
            draw_texture_ex(
                &self.building_texture,
                building.x - building.size / 2.0,
                building.y - building.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(building.size, building.size)),
                    source: Some(building_frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    // mise à jour des batiments
    pub fn update(&mut self) {
        self.buildings.retain(|building| !building.collided); // on vire les bâtiment touchés
    }

    pub fn init(&mut self) {
        for i in 0..=10 {
            self.push(Shape {
                x: 29.0+58.0*i as f32,
                y: screen_height() - 29.0,
                speed: 0.0,
                size: 58.0,
                collided: false,
            });
        }
    }
}
