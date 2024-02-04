#![no_std]

extern crate bevy;

use bevy::prelude::*;
use bevy_playdate::prelude::*;

extern crate alloc;

use {alloc::boxed::Box, anyhow::Error, bevy_playdate::*};

#[derive(Component)]
pub struct HelloWorld(ScreenPoint, ScreenVector);

fn update_hello_world(mut locations: Query<&mut HelloWorld>) {
    for mut location in locations.iter_mut() {
        let velocity = location.1.clone();
        location.0 += velocity;
        if location.0.x < 0 || location.0.x > LCD_COLUMNS as i32 - 120 {
            location.1.x *= -1;
        }
        if location.0.y < 0 || location.0.y > LCD_ROWS as i32 - 16 {
            location.1.y *= -1;
        }
    }
}

fn draw_hello_world(locations: Query<&HelloWorld>) {
    let graphics = Graphics::get();
    let _ = graphics.clear(LCDColor::Solid(crankstart_sys::LCDSolidColor::kColorWhite));
    for location in locations.iter() {
        let _ = graphics.draw_text("Hello World Bevy", location.0);
    }
    let _ = crankstart::system::System::get().draw_fps(0, 0);
}

fn spawn_hellos(mut commands: Commands) {
    commands.spawn(HelloWorld(ScreenPoint::new(0, 0), ScreenVector::new(5, 5)));
    commands.spawn(HelloWorld(
        ScreenPoint::new(0, 16),
        ScreenVector::new(5, -5),
    ));
}

struct HelloWorldGame;

impl PlaydateBevyGame for HelloWorldGame {
    fn new(app: &mut App, _playdate: &crankstart::Playdate) -> Result<Box<Self>, Error> {
        Display::get().set_refresh_rate(0.0);
        app.add_systems(Update, update_hello_world)
            .add_systems(PostUpdate, draw_hello_world)
            .add_systems(Startup, spawn_hellos);
        Ok(Box::new(Self))
    }
}

type MyGame = PlaydateBevyState<HelloWorldGame>;

crankstart_game!(MyGame);
