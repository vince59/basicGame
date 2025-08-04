mod bullet;
mod text_display;
mod enemy;

use bullet::*;
use text_display::*;
use enemy::*;
use macroquad::audio::{ PlaySoundParams, load_sound, play_sound, play_sound_once, stop_sound};
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
    const MOVEMENT_SPEED: f32 = 500.0;
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;

    let mut ship = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    let mut bullets: BulletsSet = BulletsSet::new().await;
    let mut enemies: EnemiesSet = EnemiesSet::new();

    let font = load_ttf_font("test.ttf").await.unwrap();

    let mut score: u32 = 0;
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let mut high_score: u32 = storage
        .get("highscore")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldn't load file");
    ship_texture.set_filter(FilterMode::Nearest);

    let mut ship_sprite = AnimatedSprite::new(
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

    let mut enemy_small_sprite = AnimatedSprite::new(
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

    build_textures_atlas();

    let theme_music = load_sound("8bit-spaceshooter.ogg").await.unwrap();
    let sound_laser = load_sound("laser.wav").await.unwrap();

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
                    ship.x = screen_width() / 2.0;
                    ship.y = screen_height() / 2.0;
                    score = 0;
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
                display_score(&score, &high_score);
                // dessin du vaisseau
                ship_sprite.set_animation(0);
                if is_key_down(KeyCode::Right) {
                    ship.x += ship.speed * delta_time;
                    ship_sprite.set_animation(2);
                }
                if is_key_down(KeyCode::Left) {
                    ship.x -= ship.speed * delta_time;
                    ship_sprite.set_animation(1);
                }
                if is_key_down(KeyCode::Down) {
                    ship.y += ship.speed * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    ship.y -= ship.speed * delta_time;
                }
                // on s'assure qu'on ne déborde pas de l'écran
                ship.x = clamp(ship.x, ship.size, screen_width() - ship.size);
                ship.y = clamp(ship.y, ship.size, screen_height() - ship.size);
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: ship.x,
                        y: ship.y - 24.0,
                        speed: ship.speed * 2.0,
                        size: 32.0,
                        collided: false,
                    });
                    play_sound_once(&sound_laser);
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                enemies.display(&enemy_small_sprite, &enemy_small_texture);
                // on dessine les balles
                bullets.display();
                // on dessine le vaisseau
                // test de collison entre les enemies et le vaisseau
                // affichage de game over si collison
                if enemies.get_list().iter().any(|enemy| ship.collides_with(enemy)) {
                    game_state = GameState::GameOver;
                }
                let ship_frame = ship_sprite.frame();
                draw_texture_ex(
                    &ship_texture,
                    ship.x - ship_frame.dest_size.x,
                    ship.y - ship_frame.dest_size.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(ship_frame.dest_size * 2.0),
                        source: Some(ship_frame.source_rect),
                        ..Default::default()
                    },
                );

                ship_sprite.update();
                enemy_small_sprite.update();
                bullets.update(delta_time); // on déplace les balles
                enemies.update(delta_time);
                // si il y a une collison entre une balle et un ennemi

                let mut hit = |enemy: &mut Shape| {
                    enemy.collided = true;
                    score += enemy.size.round() as u32;
                    high_score = high_score.max(score);
                };

                // pour tous les ennemis et pour toutes les balles on regarde s'il y a une collision
                for enemy in enemies.get_list() {
                    bullets.collides_with(enemy, &mut hit);
                }

                
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
                enemies.display(&enemy_small_sprite, &enemy_small_texture);

                let ship_frame = ship_sprite.frame();
                draw_texture_ex(
                    &ship_texture,
                    ship.x - ship_frame.dest_size.x,
                    ship.y - ship_frame.dest_size.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(ship_frame.dest_size * 2.0),
                        source: Some(ship_frame.source_rect),
                        ..Default::default()
                    },
                );

                display_score(&score, &high_score);
                bullets.display();
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
