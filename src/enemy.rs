/* Structure EnnemiesSet (gestion des ennemis) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig};
use macroquad_particles::{Emitter, EmitterConfig};

pub struct EnemiesSet {
    pub enemies: Vec<Shape>,
}

impl EnemiesSet {
    pub fn new() -> EnemiesSet {
        EnemiesSet {
            enemies: vec![],
        }
    }

    pub fn display(
        &self,
        enemy_small_sprite: &AnimatedSprite,
        enemy_small_texture: &Texture2D,
    ) {
        let enemy_frame = enemy_small_sprite.frame();
        for enemy in &self.enemies {
            draw_texture_ex(
                enemy_small_texture,
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(enemy.size, enemy.size)),
                    source: Some(enemy_frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    // suppression des ennemis
    pub fn clear(&mut self) {
        self.enemies.clear();
    }

    // Ajout d'une ennemi
    pub fn push(&mut self, shape: Shape) {
        self.enemies.push(shape);
    }

    // mise à jour de la positions des ennemis
    pub fn update(&mut self, delta_time: f32) {
        // ajout des ennemies : 5% de chance d'avoir un nouvel ennemie
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            self.enemies.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                collided: false,
            });
        }
        // on les fait tomber
        for enemy in &mut self.enemies {
            enemy.y += enemy.speed * delta_time;
        }
        self.enemies.retain(|enemy| enemy.y < screen_height() + enemy.size); // on vire les ennemis hors écran
        self.enemies.retain(|enemy| !enemy.collided); // on vire les ennemies touchés
    }

    pub fn get_list(&mut self) -> &mut Vec<Shape>{
        &mut self.enemies
    }

}
