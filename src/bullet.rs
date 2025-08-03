/* Structure BulletSet (gestion des balles) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::{AnimatedSprite};
use macroquad::prelude::*;
use macroquad_particles::{ Emitter, EmitterConfig};
use macroquad_particles::{self as particles, AtlasConfig};

pub struct BulletsSet {
    pub bullets: Vec<Shape>,
    pub explosions: Vec<(Emitter, Vec2)>,
    pub explosion_texture: Texture2D,
    pub sound_explosion: Sound,
}

impl BulletsSet {
    pub async fn new() -> BulletsSet {
        let explosion_texture: Texture2D = load_texture("explosion.png")
            .await
            .expect("Couldn't load file");
        explosion_texture.set_filter(FilterMode::Nearest);
        let sound_explosion = load_sound("explosion.wav").await.unwrap();
        BulletsSet {
            bullets: vec![],
            explosions: vec![],
            explosion_texture,
            sound_explosion,
        }
    }

    // suppression des balles et des explosions
    pub fn clear(&mut self) {
        self.bullets.clear();
        self.explosions.clear();
    }

    // Ajout d'une balle
    pub fn push(&mut self, shape: Shape) {
        self.bullets.push(shape);
    }

    // affichage des balles et des explosions
    pub fn display(&mut self, bullet_sprite: &AnimatedSprite, bullet_texture: &Texture2D) {
        let bullet_frame = bullet_sprite.frame();
        for bullet in &self.bullets {
            draw_texture_ex(
                bullet_texture,
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
        // dessin des explosions
        for (explosion, coords) in self.explosions.iter_mut() {
            explosion.draw(*coords);
        }
    }

    // mise à jour de la positions des balles
    pub fn update(&mut self, delta_time: f32) {
        for bullet in &mut self.bullets {
            bullet.y -= bullet.speed * delta_time;
        }
    }

    // suppression des balles et des explosions
    pub fn retain(&mut self) {
        self.bullets.retain(|bullet| !bullet.collided); // on vire les balles touchées
        self.bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0); // on vire les balles hors écran
        self.explosions.retain(|(explosion, _)| explosion.config.emitting); // on vire les explosions terminées
    }

    // test si une des balles a touché quelque chose
    pub fn collides_with<F>(&mut self, shape: &mut Shape, f: &mut F)
    where
        F: FnMut(&mut Shape),
    {
        for bullet in self.bullets.iter_mut() {
            if bullet.collides_with(&shape) {
                bullet.collided = true;
                // Ajout d'une explosion
                self.explosions.push((
                    Emitter::new(EmitterConfig {
                        amount: shape.size.round() as u32 * 4,
                        texture: Some(self.explosion_texture.clone()),
                        ..BulletsSet::particle_explosion()
                    }),
                    vec2(shape.x, shape.y),
                ));
                play_sound_once(&self.sound_explosion); // le son de l'explosion
                f(shape); // Appelle la callback pour faire d'autres choses en cas de collision
            }
        }
    }

    // retourne des valeur par défaut pour le système de particules des explosions
    fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 400.0,
        initial_velocity_randomness: 0.8,
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5, 1, 0..)),
        ..Default::default()
    }
}

}
