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
    shape: Shape,
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
                }],
                true,
            ),
        }
    }
}
pub struct BuildingsSet {
    pub buildings: Vec<Building>,
}

impl BuildingsSet {
    pub async fn new() -> BuildingsSet {
        let mut data: Vec<BuildingData> = vec![];

        let texture: Texture2D = load_texture("temple.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        let mut x = 29.;
        data.push(BuildingData {
            texture: texture,
            width: 49,
            height: 58,
            x: x,
            name: "temple".to_string(),
        });

        let texture: Texture2D = load_texture("space_port.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 90.0;
        data.push(BuildingData {
            texture: texture,
            width: 90,
            height: 58,
            x: x,
            name: "space_port".to_string(),
        });

        let texture: Texture2D = load_texture("factory.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 95.0;
        data.push(BuildingData {
            texture: texture,
            width: 90,
            height: 58,
            x: x,
            name: "factory".to_string(),
        });

        let texture: Texture2D = load_texture("labo.png").await.expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 80.0;
        data.push(BuildingData {
            texture: texture,
            width: 56,
            height: 58,
            x: x,
            name: "labo".to_string(),
        });

        let texture: Texture2D = load_texture("radio.png").await.expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 80.0;
        data.push(BuildingData {
            texture: texture,
            width: 90,
            height: 53,
            x: x,
            name: "labo".to_string(),
        });

        let texture: Texture2D = load_texture("greenhouse.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 100.0;
        data.push(BuildingData {
            texture: texture,
            width: 89,
            height: 58,
            x: x,
            name: "greenhouse".to_string(),
        });

        let texture: Texture2D = load_texture("greenhouse2.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 100.0;
        data.push(BuildingData {
            texture: texture,
            width: 89,
            height: 58,
            x: x,
            name: "greenhouse2".to_string(),
        });

        let texture: Texture2D = load_texture("greenhouse3.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 100.0;
        data.push(BuildingData {
            texture: texture,
            width: 89,
            height: 58,
            x: x,
            name: "greenhouse3".to_string(),
        });

        let texture: Texture2D = load_texture("rocket.png")
            .await
            .expect("Couldn't load file");
        texture.set_filter(FilterMode::Nearest);
        x += 100.0;
        data.push(BuildingData {
            texture: texture,
            width: 89,
            height: 104,
            x: x,
            name: "rocket".to_string(),
        });

        let mut buildings: Vec<Building> = vec![];
        for mut d in data {
            buildings.push(d.get_building());
        }
        BuildingsSet { buildings }
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
        self.buildings.retain(|building| !building.shape.collided); // on vire les bâtiment touchés
    }
}
