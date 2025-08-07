mod buildings;
mod bullets;
mod enemies;
mod explosions;
mod fires;
mod menu;
mod music;
mod score;
mod shader;
mod ship;
mod text_display;

use buildings::*;
use bullets::*;
use enemies::*;
use explosions::*;
use fires::*;
use menu::*;
use music::*;
use score::*;
use shader::*;
use ship::*;
use text_display::*;

use crate::miniquad::window::set_window_position;
use macroquad::prelude::*;

//https://vince59.github.io/basicGame/

#[derive(Clone)]
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

enum Collision {
    BulletEnemy,
    ShipEnemy,
    BuildingEnemy,
    FireEnemy,
}

fn window_conf() -> Conf {
    Conf {
        window_width: 800,  // Largeur de la fenêtre
        window_height: 800, // Hauteur de la fenêtre
        window_title: "Asteroïd".to_owned(),
        fullscreen: false, // Si tu veux que la fenêtre soit en plein écran ou non
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_window_position(10, 10);
    set_pc_assets_folder("assets");
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;
    let mut bullets = BulletsSet::new().await;
    let mut explosions = ExplosionsSet::new().await;
    let mut enemies = EnemiesSet::new().await;
    let mut buildings = BuildingsSet::new().await;
    let mut ship = Ship::new().await;
    let mut fires = FiresSet::new().await;
    let mut score = Score::new();
    let mut menu = Menu::new().await;
    build_textures_atlas();
    let font = load_ttf_font("test.ttf").await.unwrap();
    let mut starfield = Shader::new();
    let mut theme_music = Music::new().await;

    loop {
        clear_background(BLACK);
        starfield.display();
        match game_state {
            GameState::MainMenu => {
                theme_music.stop();
                let mut play = || {
                    enemies.clear();
                    bullets.clear();
                    explosions.clear();
                    buildings.reset();
                    ship.reset();
                    score.reset();
                    theme_music.reset();
                    fires.clear();
                    game_state = GameState::Playing;
                };
                menu.display(&mut play);
            }
            GameState::Playing => {
                let delta_time = get_frame_time(); // temps passé depuis la dernière frame
                // mise à jour des composants du jeux
                ship.update(delta_time);
                bullets.update(delta_time);
                explosions.update();
                enemies.update(delta_time);
                buildings.update();
                fires.update(delta_time);
                // affichages
                enemies.display();
                bullets.display();
                score.display();
                buildings.display();
                fires.display();
                explosions.display();

                if is_key_pressed(KeyCode::Space) {
                    bullets.push(ship.shoot());
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }
                let mut tmp_fires = fires.clone();
                let mut collision_handler =
                    |enemy: &mut Shape, shape: &mut Shape, collision: &Collision| {
                        match collision {
                            Collision::BulletEnemy => {
                                enemy.collided = true;
                                explosions.push(enemy); // on ajoute une explosion
                                score.increase(enemy.size.round() as u32);
                            }
                            Collision::ShipEnemy => {
                                enemy.collided = true;
                                game_state = GameState::GameOver;
                            }
                            Collision::BuildingEnemy => {
                                explosions.push(enemy); // on ajoute une explosion
                                shape.collided = true; // batiment touché
                                //fires.push(&shape); // on met un feux à la place
                            }
                            Collision::FireEnemy => {
                                explosions.push(enemy); // on ajoute une explosion
                            }
                        }
                    };

                // Collision avec une balle
                for enemy in enemies.get_list() {
                    bullets.collides_with(enemy, &Collision::BulletEnemy, &mut collision_handler);
                }

                // Collision avec un bâtiment
                for building in buildings.get_list() {
                    enemies.collides_with(
                        building,
                        &Collision::BuildingEnemy,
                        &mut collision_handler,
                    );
                }

                // collision avec un feux
                for fire in tmp_fires.get_list() {
                    enemies.collides_with(fire, &Collision::FireEnemy, &mut collision_handler);
                }

                // collision avec le vaisseau
                enemies.collides_with(
                    ship.get_shape(),
                    &Collision::ShipEnemy,
                    &mut collision_handler,
                );

                //Tous les batiments détruits ?
                if buildings.all_destroyed() {
                    game_state = GameState::GameOver;
                }
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
                buildings.display();
                fires.display();
                display_paused();
                display_game_name();
            }
            GameState::GameOver => {
                // Redémarrage du jeu si on presse espace
                if is_key_pressed(KeyCode::LeftShift) {
                    game_state = GameState::MainMenu;
                }
                display_game_over(&font);
                score.display_high_score(&font);
            }
        }
        next_frame().await
    }
}
