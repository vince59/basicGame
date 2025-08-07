/* Structure BuildingsSet (gestion des bâtiments) */

use crate::Shape;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::experimental::animation::Animation;
use macroquad::prelude::*;
use std::cmp;
pub struct BuildingData {
    pub texture: Texture2D,
    pub width: u32,
    pub height: u32,
    pub x: f32,
    pub name: String,
}

pub struct Building {
    texture: Texture2D,
    sprite: AnimatedSprite,
    pub shape: Shape,
}

impl BuildingData {
    pub fn get_building(&mut self) -> Building {
        Building {
            shape: Shape {
                x: self.x,
                y: screen_height() - (self.height / 2) as f32,
                speed: 0.0,
                size: cmp::max(self.height, self.width) as f32,
                collided: false,
            },
            texture: self.texture.clone(),
            sprite: AnimatedSprite::new(
                self.width,
                self.height,
                &[Animation {
                    name: self.name.clone(),
                    row: 0,
                    frames: 1,
                    fps: 12,
                },
                Animation {
                    name: "fire".to_string(),
                    row: 1,
                    frames: 10,
                    fps: 12,
                },],
                true,
            ),
        }
    }
}
pub struct BuildingsSet {
    pub buildings: Vec<Building>,
    pub temple_texture: Texture2D,
    pub space_port_texture: Texture2D,
    pub factory_texture: Texture2D,
    pub labo_texture: Texture2D,
    pub radio_texture: Texture2D,
    pub greenhouse_texture: Texture2D,
    pub greenhouse2_texture: Texture2D,
    pub greenhouse3_texture: Texture2D,
    pub rocket_texture: Texture2D,
}

impl BuildingsSet {
    pub async fn new() -> BuildingsSet {
        let temple_texture: Texture2D = load_texture("temple_temple_fire.png").await.expect("Couldn't load file");
        temple_texture.set_filter(FilterMode::Nearest);

        let space_port_texture: Texture2D = load_texture("space_port_space_port_fire.png").await.expect("Couldn't load file");
        space_port_texture.set_filter(FilterMode::Nearest);

        let factory_texture: Texture2D = load_texture("factory_factory_fire.png").await.expect("Couldn't load file");
        factory_texture.set_filter(FilterMode::Nearest);

        let labo_texture: Texture2D = load_texture("labo_labo_fire.png").await.expect("Couldn't load file");
        labo_texture.set_filter(FilterMode::Nearest);

        let radio_texture: Texture2D = load_texture("radio_radio_fire.png").await.expect("Couldn't load file");
        radio_texture.set_filter(FilterMode::Nearest);

        let greenhouse_texture: Texture2D = load_texture("greenhouse_greenhouse_fire.png").await.expect("Couldn't load file");
        greenhouse_texture.set_filter(FilterMode::Nearest);

        let greenhouse2_texture: Texture2D = load_texture("greenhouse2_greenhouse2_fire.png").await.expect("Couldn't load file");
        greenhouse2_texture.set_filter(FilterMode::Nearest);

        let greenhouse3_texture: Texture2D = load_texture("greenhouse3_greenhouse3_fire.png").await.expect("Couldn't load file");
        greenhouse3_texture.set_filter(FilterMode::Nearest);

        let rocket_texture: Texture2D = load_texture("rocket_rocket_fire.png").await.expect("Couldn't load file");
        rocket_texture.set_filter(FilterMode::Nearest);

        let buildings: Vec<Building> = vec![];
        BuildingsSet {
            buildings,
            temple_texture,
            space_port_texture,
            factory_texture,
            labo_texture,
            radio_texture,
            greenhouse_texture,
            greenhouse2_texture,
            greenhouse3_texture,
            rocket_texture,
        }
    }

    pub fn reset(&mut self) {
        let mut data: Vec<BuildingData> = vec![];
        let mut x = 29.;
         data.push(BuildingData {
            texture: self.temple_texture.clone(),
            width: 86,
            height: 80,
            x: x,
            name: "temple".to_string(),
        });

        x += 90.0;
        data.push(BuildingData {
            texture: self.space_port_texture.clone(),
            width: 90,
            height: 58,
            x: x,
            name: "space_port".to_string(),
        });
        
        x += 95.0;
        data.push(BuildingData {
            texture: self.factory_texture.clone(),
            width: 90,
            height: 58,
            x: x,
            name: "factory".to_string(),
        });
        
        x += 80.0;
        data.push(BuildingData {
            texture: self.labo_texture.clone(),
            width: 92,
            height: 67,
            x: x,
            name: "labo".to_string(),
        });

        x += 80.0;
        data.push(BuildingData {
            texture: self.radio_texture.clone(),
            width: 90,
            height: 53,
            x: x,
            name: "radio".to_string(),
        });

        x += 100.0;
        data.push(BuildingData {
            texture: self.greenhouse_texture.clone(),
            width: 89,
            height: 58,
            x: x,
            name: "greenhouse".to_string(),
        });

        x += 100.0;
        data.push(BuildingData {
            texture: self.greenhouse2_texture.clone(),
            width: 88,
            height: 58,
            x: x,
            name: "greenhouse2".to_string(),
        });

        x += 100.0;
        data.push(BuildingData {
            texture: self.greenhouse3_texture.clone(),
            width: 89,
            height: 58,
            x: x,
            name: "greenhouse3".to_string(),
        });

        x += 100.0;
        data.push(BuildingData {
            texture: self.rocket_texture.clone(),
            width: 89,
            height: 104,
            x: x,
            name: "rocket".to_string(),
        });
        
        for mut d in data {
            self.buildings.push(d.get_building());
        }
    }

    // affichage des bâtiments
    pub fn display(&mut self) {
        for building in &self.buildings {
            let building_frame = building.sprite.frame();
            draw_texture_ex(
                &building.texture,
                building.shape.x - building.shape.size / 2.0,
                building.shape.y - building.shape.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(building.shape.size, building.shape.size)),
                    source: Some(building_frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    pub fn get_list(&mut self) -> Vec<&mut Shape> {
        self.buildings
            .iter_mut()
            .map(|building| &mut building.shape)
            .collect()
    }

    // mise à jour des batiments
    pub fn update(&mut self) {
        for building in &mut self.buildings {
            if building.shape.collided {
                building.sprite.set_animation(1);
            } else {
                building.sprite.set_animation(0);
            }
            building.sprite.update();
        }
    }

    pub fn all_destroyed(&mut self) -> bool {
        let mut destroyed= true;
        for building in &mut self.buildings {
            if building.shape.collided {
                building.sprite.set_animation(1);
            } else {
                building.sprite.set_animation(0);
                destroyed = false;
            }
            building.sprite.update();
        }
        destroyed
    }
}
