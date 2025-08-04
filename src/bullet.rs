/* Structure BulletSet (gestion des balles) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig};
use macroquad_particles::{Emitter, EmitterConfig};
use macroquad::experimental::animation::{Animation};

pub struct BulletsSet {
    pub bullets: Vec<Shape>,
    pub bullet_texture: Texture2D,
    pub explosions: Vec<(Emitter, Vec2)>,
    pub explosion_texture: Texture2D,
    pub sound_explosion: Sound,
    pub bullet_sprite: AnimatedSprite
}

impl BulletsSet {
    pub async fn new() -> BulletsSet {
        let explosion_texture: Texture2D = load_texture("explosion.png")
            .await
            .expect("Couldn't load file");
        explosion_texture.set_filter(FilterMode::Nearest);
        let bullet_texture: Texture2D = load_texture("laser-bolts.png")
            .await
            .expect("Couldn't load file");
        bullet_texture.set_filter(FilterMode::Nearest);
        let sound_explosion = load_sound("explosion.wav").await.unwrap();
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
            explosions: vec![],
            explosion_texture,
            sound_explosion,
            bullet_sprite
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
        self.bullets.retain(|bullet| !bullet.collided); // on vire les balles touchées
        self.bullets
            .retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0); // on vire les balles hors écran
        self.explosions
            .retain(|(explosion, _)| explosion.config.emitting); // on vire les explosions terminées
        self.bullet_sprite.update();
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
