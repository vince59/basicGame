mod bullet;

use bullet::*;

use macroquad::audio::{
    PlaySoundParams, Sound, load_sound, play_sound, play_sound_once, stop_sound,
};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, Emitter, EmitterConfig};

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

pub fn display_game_over(font: &Font) {
    let text = "GAME OVER!";
    let text_params = TextParams {
        font_size: 50,
        font: Some(font),
        color: RED,
        font_scale: 1.0,
        ..Default::default()
    };
    let text_dimensions = measure_text(
        text,
        text_params.font,
        text_params.font_size,
        text_params.font_scale,
    );
    draw_text_ex(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0 + text_dimensions.height / 2.0,
        text_params,
    );
}

pub fn display_congratulations(font: &Font) {
    let text = "CONGRATULATIONS ! you reached a high score";
    let text_params = TextParams {
        font_size: 25,
        font: Some(font),
        color: YELLOW,
        font_scale: 1.0,
        ..Default::default()
    };
    let text_dimensions = measure_text(
        text,
        text_params.font,
        text_params.font_size,
        text_params.font_scale,
    );
    draw_text_ex(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0 + text_dimensions.height / 2.0 + 60.0,
        text_params,
    );
}

pub fn display_score(score: &u32, high_score: &u32) {
    draw_text(
        format!("Score: {}", score).as_str(),
        10.0,
        35.0,
        25.0,
        WHITE,
    );
    let highscore_text = format!("High score: {}", high_score);
    let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
    draw_text(
        highscore_text.as_str(),
        screen_width() - text_dimensions.width - 10.0,
        35.0,
        25.0,
        WHITE,
    );
}

pub fn display_enemies(
    enemies: &Vec<Shape>,
    enemy_small_sprite: &AnimatedSprite,
    enemy_small_texture: &Texture2D,
) {
    let enemy_frame = enemy_small_sprite.frame();
    for enemy in enemies {
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

pub fn display_paused() {
    let text = "Paused";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        WHITE,
    );
}

pub fn display_game_name() {
    let text = "Asteroïd";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        text_dimensions.height + 10.0,
        50.0,
        YELLOW,
    );
}

pub fn display_press_space() {
    let text = "Press space";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        WHITE,
    );
}

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

    pub fn clear(&mut self) {
        self.bullets.clear();
        self.explosions.clear();
    }

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
        self.explosions
            .retain(|(explosion, _)| explosion.config.emitting);
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
                        ..particle_explosion()
                    }),
                    vec2(shape.x, shape.y),
                ));
                play_sound_once(&self.sound_explosion); // le son de l'explosion
                f(shape); // Appelle la callback 
            }
        }
    }
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
    let mut enemies: Vec<Shape> = vec![];
    let mut bullets: BulletsSet = BulletsSet::new().await;

    let font = load_ttf_font("test.ttf").await.unwrap();

    let mut score: u32 = 0;
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let mut high_score: u32 = storage
        .get("highscore")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    //let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldn't load file");
    ship_texture.set_filter(FilterMode::Nearest);
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

    /*let explosion_texture: Texture2D = load_texture("explosion.png")
        .await
        .expect("Couldn't load file");
    explosion_texture.set_filter(FilterMode::Nearest);*/

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
    test();
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
                    //explosions.clear();
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

                // on dessine les balles
                bullets.display(&bullet_sprite, &bullet_texture);
                // on dessine le vaisseau
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
                bullet_sprite.update();
                enemy_small_sprite.update();

                // ajout des ennemies : 5% de chance d'avoir un nouvel ennemie
                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    enemies.push(Shape {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                        y: -size,
                        collided: false,
                    });
                }
                // on les fait tomber
                for enemy in &mut enemies {
                    enemy.y += enemy.speed * delta_time;
                }
                enemies.retain(|enemy| enemy.y < screen_height() + enemy.size); // on vire les ennemie hors écran

                // on déplace les balles
                bullets.update(delta_time);

                // si il y a une collison entre une balle et un ennemie

                let mut hit = |enemy: &mut Shape| {
                    enemy.collided = true;
                    score += enemy.size.round() as u32;
                    high_score = high_score.max(score);
                };

                // pour tous les ennemies et pour toutes les balles on regarde s'il y a une collision
                for enemy in enemies.iter_mut() {
                    bullets.collides_with(enemy, &mut hit);
                }

                bullets
                    .bullets
                    .retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0); // on vire les balles hors écran
                enemies.retain(|enemy| !enemy.collided); // on vire les ennemies touchés
                bullets.retain(); // on vire les balles touchées

                // on dessine les ennemies
                display_enemies(&enemies, &enemy_small_sprite, &enemy_small_texture);

                // test de collison entre les enemies et le vaisseau
                // affichage de game over si collison
                if enemies.iter().any(|enemy| ship.collides_with(enemy)) {
                    game_state = GameState::GameOver;
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
                display_enemies(&enemies, &enemy_small_sprite, &enemy_small_texture);

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
                bullets.display(&bullet_sprite, &bullet_texture);
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
