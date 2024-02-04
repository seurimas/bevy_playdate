#![no_std]

use bevy::{prelude::*, utils::Instant};
#[cfg(cortex_m)]
use cortex_m as _;
use rand::{Rng, SeedableRng};
use sprites::PdSpritePlugin;

extern crate alloc;
extern crate bevy;
extern crate getrandom;

pub mod prelude;
pub mod sprites;

use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{Game, Playdate},
};

pub trait PlaydateBevyGame {
    fn new(app: &mut App, _playdate: &Playdate) -> Result<Box<Self>, Error>;
}

pub struct PlaydateBevyState<G: PlaydateBevyGame>(App, G);

impl<G: PlaydateBevyGame> PlaydateBevyState<G> {
    pub fn new_app(mut app: App, _playdate: &Playdate) -> Result<Box<Self>, Error> {
        G::new(&mut app, _playdate).map(|game| Box::new(Self(app, *game)))
    }

    pub fn new(playdate: &Playdate) -> Result<Box<Self>, Error> {
        Instant::register(playdate.playdate);
        let app = App::new();
        Self::new_app(app, playdate)
    }
}

impl<G: PlaydateBevyGame> Game for PlaydateBevyState<G> {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        self.0.update();

        Ok(())
    }

    fn update_sprite(
        &mut self,
        _sprite: &mut crankstart::sprite::Sprite,
        _playdate: &mut Playdate,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(not(cortex_m))]
mod bad_critical_section {
    use critical_section::{set_impl, Impl, RawRestoreState};

    struct SingleCoreCriticalSection;
    set_impl!(SingleCoreCriticalSection);

    unsafe impl Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            false
        }

        unsafe fn release(was_active: RawRestoreState) {
            // We're really dumb.
        }
    }
}

const SEED_MASK: u64 = 0xdeadbeefbadc0ded;

fn getrandom_seeded(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    let seconds = crankstart::system::System::get()
        .get_seconds_since_epoch()
        .unwrap();
    let seed = seconds.1 as u64 + (seconds.0 as u64) << 32;
    let seed = SEED_MASK ^ seed;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    rng.fill(dest);
    Ok(())
}

getrandom::register_custom_getrandom!(getrandom_seeded);
