mod bullets;
mod enemies;
mod menu;
mod music;
mod score;
mod shader;
mod ship;
mod text_display;
mod buildings;

use bullets::*;
use enemies::*;
use menu::*;
use music::*;
use score::*;
use shader::*;
use ship::*;
use text_display::*;
use buildings::*;

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

fn window_conf() -> Conf {
    Conf {
        window_width: 800,   // Largeur de la fenêtre
        window_height: 600,  // Hauteur de la fenêtre
        window_title: "Asteroïd".to_owned(),
        fullscreen: false,   // Si tu veux que la fenêtre soit en plein écran ou non
        ..Conf::default() 
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("{} {}",screen_width(), screen_height());
    set_pc_assets_folder("assets");
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;
    let mut bullets = BulletsSet::new().await;
    let mut enemies = EnemiesSet::new().await;
    let mut buildings = BuildingsSet::new().await;
    let mut ship = Ship::new().await;
    let mut score = Score::new();
    let mut menu = Menu::new().await;
    build_textures_atlas();
    let font = load_ttf_font("test.ttf").await.unwrap();
    let mut starfield = Shader::new();
    let mut theme_music = Music::new().await;
    buildings.init();
    loop {
        clear_background(BLACK);
        starfield.display();
        buildings.display();
        match game_state {
                GameState::MainMenu => {
                theme_music.stop();
                let mut play = || {
                    enemies.clear();
                    bullets.clear();
                    ship.reset();
                    score.reset();
                    theme_music.reset();
                    game_state = GameState::Playing;
                };
                menu.display(&mut play);
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
                score.display();

                if is_key_pressed(KeyCode::Space) {
                    bullets.push(ship.shoot());
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // si il y a une collison entre une balle et un ennemi
                let mut hit_enemy_bullet = |enemy: &mut Shape| {
                    enemy.collided = true;
                    score.increase(enemy.size.round() as u32);
                };

                // s'il y a une collision entre un ennemi et le vaisseau
                let mut hit_ship_enemy = |ship: &mut Shape| {
                    ship.collided = true;
                    game_state = GameState::GameOver;
                };

                // Vérification des collisions
                for enemy in enemies.get_list() {
                    bullets.collides_with(enemy, &mut hit_enemy_bullet); // collision avec une balle
                }
                enemies.collides_with(ship.get_shape(), &mut hit_ship_enemy); // collision avec le vaisseau
            }
            GameState::Paused => {
                theme_music.stop();
                if is_key_pressed(KeyCode::Space) {
                    theme_music.play();
                    game_state = GameState::Playing;
                }
                enemies.display();
                ship.display();
                bullets.display();
                score.display();
                display_paused();
                display_game_name();
            }
            GameState::GameOver => {
                // Redémarrage du jeu si on presse espace
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                display_game_over(&font);
                score.display_high_score(&font);
            }
        }
        next_frame().await
    }
}
