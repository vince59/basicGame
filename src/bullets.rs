/* Structure BulletSet (gestion des balles) */

use crate::{Collision, Shape};
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::experimental::animation::Animation;
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig};
use macroquad_particles::{Emitter, EmitterConfig};

pub struct BulletsSet {
    pub bullets: Vec<Shape>,
    pub bullet_texture: Texture2D,
    pub bullet_sprite: AnimatedSprite,
}

impl BulletsSet {
    pub async fn new() -> BulletsSet {
        let bullet_texture: Texture2D = load_texture("laser-bolts.png")
            .await
            .expect("Couldn't load file");
        bullet_texture.set_filter(FilterMode::Nearest);
        let mut bullet_sprite = AnimatedSprite::new(
            16,
            16,
            &[
                Animation {
                    name: "bullet".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "bolt".to_string(),
                    row: 1,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        );
        bullet_sprite.set_animation(1);
        BulletsSet {
            bullets: vec![],
            bullet_texture,
            bullet_sprite,
        }
    }

    // suppression des balles et des explosions
    pub fn clear(&mut self) {
        self.bullets.clear();
    }

    // Ajout d'une balle
    pub fn push(&mut self, shape: Shape) {
        self.bullets.push(shape);
    }

    // affichage des balles et des explosions
    pub fn display(&mut self) {
        let bullet_frame = self.bullet_sprite.frame();
        for bullet in &self.bullets {
            draw_texture_ex(
                &self.bullet_texture,
                bullet.x - bullet.size / 2.0,
                bullet.y - bullet.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bullet.size, bullet.size)),
                    source: Some(bullet_frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    // mise à jour de la positions des balles
    pub fn update(&mut self, delta_time: f32) {
        for bullet in &mut self.bullets {
            bullet.y -= bullet.speed * delta_time;
        }
        self.bullets.retain(|bullet| !bullet.collided); // on vire les balles touchées
        self.bullets
            .retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0); // on vire les balles hors écran
        self.bullet_sprite.update();
    }

    // test si une des balles a touché quelque chose
    pub fn collides_with<F>(&mut self, shape: &mut Shape, collision: &Collision, f: &mut F)
    where
        F: FnMut(&mut Shape, &mut Shape, &Collision),
    {
        for bullet in self.bullets.iter_mut() {
            if bullet.collides_with(&shape) {
                bullet.collided = true;
                f(shape, bullet, collision); // Appelle la callback pour faire d'autres choses en cas de collision
            }
        }
    }
}
