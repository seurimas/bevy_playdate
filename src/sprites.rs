use crate::prelude::*;
use bevy::prelude::*;
use crankstart::{sprite::SpriteManager, system::System};

use alloc::format;

#[derive(Component, Deref, DerefMut)]
pub struct PdSprite(Sprite);

// XXX: This is not actually safe, but it's the best I can do for now.
unsafe impl Send for PdSprite {}
unsafe impl Sync for PdSprite {}

impl Default for PdSprite {
    fn default() -> Self {
        PdSprite(SpriteManager::get_mut().new_sprite().unwrap())
    }
}

#[derive(Bundle, Default)]
pub struct PdSpriteBundle {
    pub sprite: PdSprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub struct PdSpritePlugin;

impl Plugin for PdSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (add_sprites_to_draw, update_sprites));
    }
}

fn add_sprites_to_draw(query: Query<&PdSprite, (With<GlobalTransform>, Added<PdSprite>)>) {
    for sprite in query.iter() {
        if let Err(err) = SpriteManager::get_mut().add_sprite(&sprite.0) {
            System::log_to_console(&format!("{:?}", err));
        }
    }
}

fn update_sprites(
    mut query: Query<(&mut PdSprite, &GlobalTransform), Or<(Changed<Transform>, Added<Transform>)>>,
) {
    for (mut sprite, global_transform) in query.iter_mut() {
        if let Err(err) = sprite.move_to(
            global_transform.translation().x,
            global_transform.translation().y,
        ) {
            System::log_to_console(&format!("{:?}", err));
        }
        System::log_to_console(&format!("{:?}", sprite.0.get_position()));
    }
}
