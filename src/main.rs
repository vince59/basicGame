mod bullet;
mod enemy;
mod ship;
mod text_display;

use bullet::*;
use enemy::*;
use ship::*;
use text_display::*;

use macroquad::audio::{PlaySoundParams, load_sound, play_sound, play_sound_once, stop_sound};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

//https://vince59.github.io/basicGame/

pub struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }
    fn circle(&self) -> Circle {
        Circle {
            x: (self.x),
            y: (self.y),
            r: (self.size),
        }
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("Astéroïd")]
async fn main() {
    set_pc_assets_folder("assets");
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;
    let mut bullets: BulletsSet = BulletsSet::new().await;
    let mut enemies: EnemiesSet = EnemiesSet::new().await;
    let mut ship: Ship = Ship::new().await;

    let font = load_ttf_font("test.ttf").await.unwrap();

    let mut score: u32 = 0;
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let mut high_score: u32 = storage
        .get("highscore")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    build_textures_atlas();

    let theme_music = load_sound("8bit-spaceshooter.ogg").await.unwrap();

    let img = Image::gen_image_color(1, 1, WHITE);
    let texture = Texture2D::from_image(&img);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("time", UniformType::Float1),
                UniformDesc::new("screen_size", UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .unwrap();
    loop {
        clear_background(BLACK);

        gl_use_material(&material);
        material.set_uniform("time", get_time() as f32);
        material.set_uniform("screen_size", vec2(screen_width(), screen_height()));
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();
        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    enemies.clear();
                    bullets.clear();
                    ship.reset();
                    score = 0;
                    stop_sound(&theme_music);
                    play_sound(
                        &theme_music,
                        PlaySoundParams {
                            looped: true,
                            volume: 1.,
                        },
                    );
                    game_state = GameState::Playing;
                }
                display_press_space();
            }
            GameState::Playing => {
                let delta_time = get_frame_time(); // temps passé depuis la dernière frame
                
                // mise à jour des composants du jeux
                ship.update(delta_time);
                bullets.update(delta_time); 
                enemies.update(delta_time);

                // affichages
                enemies.display();
                bullets.display();
                display_score(&score, &high_score);

                if is_key_pressed(KeyCode::Space) {
                    bullets.push(ship.shoot());
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }
                
                // si il y a une collison entre une balle et un ennemi
                let mut hit_enemy_bullet = |enemy: &mut Shape| {
                    enemy.collided = true;
                    score += enemy.size.round() as u32;
                    high_score = high_score.max(score);
                };

                // s'il y a une collision entre un ennemi et le vaisseau
                let mut hit_ship_enemy = |ship: &mut Shape| {
                    ship.collided=true;
                    game_state = GameState::GameOver;
                };

                // Vérification des collisions
                for enemy in enemies.get_list() {
                    bullets.collides_with(enemy, &mut hit_enemy_bullet); // collision avec une balle
                }
                enemies.collides_with(ship.get_shape(), &mut hit_ship_enemy); // collision avec le vaisseau

            }
            GameState::Paused => {
                stop_sound(&theme_music);
                if is_key_pressed(KeyCode::Space) {
                    play_sound(
                        &theme_music,
                        PlaySoundParams {
                            looped: true,
                            volume: 1.,
                        },
                    );
                    game_state = GameState::Playing;
                }
                enemies.display();
                ship.display();
                bullets.display();

                display_score(&score, &high_score);
                display_paused();
                display_game_name();
            }
            GameState::GameOver => {
                // Redémarrage du jeu si on presse espace
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                display_game_over(&font);
                if score == high_score {
                    let s = high_score.to_string();
                    storage.set("highscore", &s);
                    display_congratulations(&font);
                }
            }
        }
        next_frame().await
    }
}

const VERTEX_SHADER: &str = r#" #version 100
attribute vec3 position;
attribute vec2 texcoord;

uniform vec2 screen_size;
varying vec2 uv;

void main() {
    uv = texcoord;

    // Convertit la position (en pixels) en clip-space [-1, 1]
    vec2 clip_pos = (position.xy / screen_size) * 2.0 - 1.0;
    clip_pos.y = -clip_pos.y; // Inversion verticale pour OpenGL

    gl_Position = vec4(clip_pos, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#" #version 100
precision mediump float;

varying vec2 uv;
uniform float time;

// Fonction pseudo-aléatoire
float random(vec2 st) {
    return fract(sin(dot(st, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    // Crée une grille virtuelle 100x100
    vec2 cell = floor(uv * 200.0);

    float r = random(cell);

    if (r > 0.99) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0); // Pixel blanc
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Fond noir
    }
}
"#;
