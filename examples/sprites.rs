#![no_std]

extern crate bevy;

use bevy::{prelude::*, time::TimePlugin, utils::Instant};
use bevy_playdate::prelude::*;
use bevy_playdate::sprites::PdSpritePlugin;
use crankstart::graphics::LCDColor;
use crankstart::sprite::SpriteManager;
use euclid::Size2D;

extern crate alloc;

use {alloc::boxed::Box, alloc::format, anyhow::Error, bevy_playdate::*};

#[derive(Component)]
pub struct Ball(ScreenVector);

fn update_balls(mut locations: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in locations.iter_mut() {
        let velocity = ball.0;
        transform.translation.x += velocity.x as f32;
        transform.translation.y += velocity.y as f32;
        if (transform.translation.x < 8.0 && ball.0.x < 0)
            || (transform.translation.x > LCD_COLUMNS as f32 - 8.0 && ball.0.x > 0)
        {
            ball.0.x *= -1;
        }
        if (transform.translation.y < 8.0 && ball.0.y < 0)
            || (transform.translation.y > LCD_ROWS as f32 - 8.0 && ball.0.y > 0)
        {
            ball.0.y *= -1;
        }
    }
}

fn spawn_ball(mut commands: Commands) {
    SpriteManager::get_mut().new_sprite().unwrap();
    let image = Graphics::get()
        .new_bitmap(
            Size2D::new(16, 16),
            LCDColor::Solid(crankstart_sys::LCDSolidColor::kColorBlack),
        )
        .unwrap();
    let mut sprite = PdSprite::default();
    let _ = sprite.set_image(image, crankstart_sys::LCDBitmapFlip::kBitmapUnflipped);
    PdSystem::log_to_console("Here!");
    let sprite = PdSpriteBundle {
        sprite,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        global_transform: GlobalTransform::default(),
    };
    commands.spawn((Ball(ScreenVector::new(5, 5)), sprite));
}

fn draw_fps(time: Res<Time>) {
    PdSystem::log_to_console(&format!("{}", 1.0 / time.delta_seconds()));
    PdSystem::log_to_console(&format!("{:?}", Instant::now()));
    if let Err(err) = crankstart::system::System::get().draw_fps(0, 0) {
        PdSystem::log_to_console(&format!("{:?}", err));
    }
}

struct SpriteGame;

impl PlaydateBevyGame for SpriteGame {
    fn new(app: &mut App, _playdate: &crankstart::Playdate) -> Result<Box<Self>, Error> {
        app.add_plugins((TransformPlugin, PdSpritePlugin, TimePlugin));
        app.add_systems(Update, update_balls)
            .add_systems(PostUpdate, draw_fps)
            .add_systems(Startup, spawn_ball);
        Ok(Box::new(Self))
    }
}

type MyGame = PlaydateBevyState<SpriteGame>;

crankstart_game!(MyGame);
