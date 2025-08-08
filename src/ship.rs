/* Structure ShipSet (gestion du vaisseau) */

use crate::Shape;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::experimental::animation::Animation;
use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 500.0;
const NB_LIFE: i32 = 5;
const NB_AMMO: i32 = 100;

pub struct Ship {
    pub ship: Shape,
    pub ship_sprite: AnimatedSprite,
    pub sound_laser: Sound,
    ship_texture: Texture2D,
    heart_texture: Texture2D,
    heart2_texture: Texture2D,
    ammo_texture: Texture2D,
    ammo2_texture: Texture2D,
    pub nb_ammo:i32
}

impl Ship {
    pub async fn new() -> Ship {
        let ship = Shape {
            size: 16.0,
            speed: MOVEMENT_SPEED,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            collided: false,
            life: NB_LIFE
        };
        let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldn't load file");
        let heart_texture = load_texture("heart.png").await.expect("Couldn't load file");
        let heart2_texture = load_texture("heart2.png")
            .await
            .expect("Couldn't load file");
        let ammo_texture = load_texture("ammo.png").await.expect("Couldn't load file");
        let ammo2_texture = load_texture("ammo2.png")
            .await
            .expect("Couldn't load file");
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
        let nb_ammo=NB_AMMO;

        Ship {
            ship,
            ship_sprite,
            sound_laser,
            ship_texture,
            heart_texture,
            heart2_texture,
            ammo_texture,
            ammo2_texture,
            nb_ammo
        }
    }

    pub fn reset(&mut self) {
        self.ship.x = screen_width() / 2.0;
        self.ship.y = screen_height() / 2.0;
    }

    fn display_life(&self){
        let start_x = 180.0;
        let y = 20.0;
        let spacing = 20.0;

        for i in 0..NB_LIFE {
            let x = start_x + (i as f32 * spacing);
            let texture = if i < self.ship.life {
                &self.heart_texture
            } else {
                &self.heart2_texture
            };
            draw_texture(texture, x, y, WHITE);
        }
    }

    fn display_ammo(&self){
        let start_x = 182.0;
        let y = 40.0;
        let spacing = 22.0;

        for i in 0..NB_AMMO/20 {
            let x = start_x + (i as f32 * spacing);
            let texture = if i < self.nb_ammo/20 {
                &self.ammo_texture
            } else {
                &self.ammo2_texture
            };
            draw_texture(texture, x, y, WHITE);
        }
    }

    pub fn display(&self) {
        self.display_life();
        self.display_ammo();
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
            screen_height() - self.ship.size - 110.0,
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
            life: 0,
        }
    }

    pub fn get_shape(&mut self) -> &mut Shape {
        &mut self.ship
    }
}
