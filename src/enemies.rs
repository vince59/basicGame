/* Structure EnnemiesSet (gestion des ennemis) */

use crate::{Collision, Shape};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

pub struct EnemiesSet {
    pub enemies: Vec<Shape>,
    enemy_small_sprite: AnimatedSprite,
    enemy_medium_sprite: AnimatedSprite,
    enemy_big_sprite: AnimatedSprite,
    enemy_small_texture: Texture2D,
    enemy_medium_texture: Texture2D,
    enemy_big_texture: Texture2D,
}

impl EnemiesSet {
    pub async fn new() -> EnemiesSet {
        let enemy_small_sprite = AnimatedSprite::new(
            17,
            16,
            &[Animation {
                name: "enemy_small".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true,
        );
        let enemy_medium_sprite = AnimatedSprite::new(
            32,
            16,
            &[Animation {
                name: "enemy_medium".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true,
        );
        let enemy_big_sprite = AnimatedSprite::new(
            32,
            32,
            &[Animation {
                name: "enemy_big".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true,
        );
        let enemy_small_texture: Texture2D = load_texture("enemy-small.png")
            .await
            .expect("Couldn't load file");
        enemy_small_texture.set_filter(FilterMode::Nearest);

        let enemy_medium_texture: Texture2D = load_texture("enemy-medium.png")
            .await
            .expect("Couldn't load file");
        enemy_medium_texture.set_filter(FilterMode::Nearest);

        let enemy_big_texture: Texture2D = load_texture("enemy-big.png")
            .await
            .expect("Couldn't load file");
        enemy_big_texture.set_filter(FilterMode::Nearest);

        EnemiesSet {
            enemies: vec![],
            enemy_small_texture,
            enemy_medium_texture,
            enemy_big_texture,
            enemy_small_sprite,
            enemy_medium_sprite,
            enemy_big_sprite
        }
    }

    pub fn display(&self) {
        for enemy in &self.enemies {
            let (texture,frame) = match enemy.size {
                _ if enemy.size <= 32.0 => (&self.enemy_small_texture,self.enemy_small_sprite.frame()),
                33.0..=48.0 => (&self.enemy_medium_texture,self.enemy_medium_sprite.frame()),
                _ => (&self.enemy_big_texture,self.enemy_big_sprite.frame()),
            };
            draw_texture_ex(
                &texture,
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(enemy.size, enemy.size)),
                    source: Some(frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    // suppression des ennemis
    pub fn clear(&mut self) {
        self.enemies.clear();
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
        self.enemies
            .retain(|enemy| enemy.y < screen_height() + enemy.size); // on vire les ennemis hors écran
        self.enemies.retain(|enemy| !enemy.collided); // on vire les ennemies touchés
        self.enemy_small_sprite.update();
    }

    pub fn get_list(&mut self) -> &mut Vec<Shape> {
        &mut self.enemies
    }

    pub fn collides_with<F>(&mut self, shape: &mut Shape, collision: &Collision, f: &mut F)
    where
        F: FnMut(&mut Shape,&mut Shape, &Collision),
    {
        for enemy in self.enemies.iter_mut() {
            if enemy.collides_with(&shape) {
                enemy.collided = true;
                f(enemy,shape,collision); // Appelle la callback pour faire d'autres choses en cas de collision
            }
        }
    }
}
