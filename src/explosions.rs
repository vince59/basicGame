/* Structure BulletSet (gestion des balles) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig};
use macroquad_particles::{Emitter, EmitterConfig};

pub struct ExplosionsSet {
    pub explosions: Vec<(Emitter, Vec2)>,
    pub explosion_texture: Texture2D,
    pub sound_explosion: Sound,
}

impl ExplosionsSet {
    pub async fn new() -> ExplosionsSet {
        let explosion_texture: Texture2D = load_texture("explosion.png")
            .await
            .expect("Couldn't load file");
        explosion_texture.set_filter(FilterMode::Nearest);
        let sound_explosion = load_sound("explosion.wav").await.unwrap();
        ExplosionsSet {
            explosions: vec![],
            explosion_texture,
            sound_explosion,
        }
    }

    // suppression des explosions
    pub fn clear(&mut self) {
        self.explosions.clear();
    }

    // Ajout d'une explosion
    pub fn push(&mut self, shape: &Shape) {
        
        self.explosions.push((
            Emitter::new(EmitterConfig {
                amount: shape.size.round() as u32 * 4,
                texture: Some(self.explosion_texture.clone()),
                ..ExplosionsSet::particle_explosion()
            }),
            vec2(shape.x, shape.y),
        ));
        play_sound_once(&self.sound_explosion); // le son de l'explosion
    }

    // affichage des explosions
    pub fn display(&mut self) {
        for (explosion, coords) in self.explosions.iter_mut() {
            explosion.draw(*coords);
        }
    }

    // mise à jour de la positions des balles
    pub fn update(&mut self) {
        self.explosions
            .retain(|(explosion, _)| explosion.config.emitting); // on vire les explosions terminées
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
