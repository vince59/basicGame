/* Structure ShipSet (gestion du vaisseau) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::experimental::animation::Animation;
use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 500.0;

pub struct Ship {
    pub ship: Shape,
    pub ship_sprite: AnimatedSprite,
    pub sound_laser: Sound,
    ship_texture: Texture2D,
}

impl Ship {
    pub async fn new() -> Ship {
        let ship = Shape {
            size: 16.0,
            speed: MOVEMENT_SPEED,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            collided: false,
        };
        let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldn't load file");
        ship_texture.set_filter(FilterMode::Nearest);

        let ship_sprite = AnimatedSprite::new(
            16,
            24,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "left".to_string(),
                    row: 2,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "right".to_string(),
                    row: 4,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        );

        let sound_laser = load_sound("laser.wav").await.unwrap();

        Ship {
            ship,
            ship_sprite,
            sound_laser,
            ship_texture,
        }
    }

    pub fn reset(&mut self) {
        self.ship.x = screen_width() / 2.0;
        self.ship.y = screen_height() / 2.0;
    }

    pub fn display(&self) {
        let ship_frame = self.ship_sprite.frame();
        draw_texture_ex(
            &self.ship_texture,
            self.ship.x - ship_frame.dest_size.x,
            self.ship.y - ship_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ship_frame.dest_size * 2.0),
                source: Some(ship_frame.source_rect),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, delta_time: f32) {
        self.ship_sprite.set_animation(0);
        if is_key_down(KeyCode::Right) {
            self.ship.x += self.ship.speed * delta_time;
            self.ship_sprite.set_animation(2);
        }
        if is_key_down(KeyCode::Left) {
            self.ship.x -= self.ship.speed * delta_time;
            self.ship_sprite.set_animation(1);
        }
        if is_key_down(KeyCode::Down) {
            self.ship.y += self.ship.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            self.ship.y -= self.ship.speed * delta_time;
        }
        // on s'assure qu'on ne déborde pas de l'écran
        self.ship.x = clamp(self.ship.x, self.ship.size, screen_width() - self.ship.size);
        self.ship.y = clamp(
            self.ship.y,
            self.ship.size,
            screen_height() - self.ship.size,
        );
        self.display();
        self.ship_sprite.update();
    }

    pub fn shoot(&self) -> Shape {
        play_sound_once(&self.sound_laser);
        Shape {
            x: self.ship.x,
            y: self.ship.y - 24.0,
            speed: self.ship.speed * 2.0,
            size: 32.0,
            collided: false,
        }
    }

    pub fn get_shape(&mut self) -> &mut Shape{
        &mut self.ship
    }
}
