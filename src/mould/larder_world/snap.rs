//! **Save and resume A** (Milestone 2, Direction 2026-11-24): the whole
//! game mid-planet, written out and read back. Step C (the pause menu's
//! line, #748) relanded the planet fresh from the run save; this keeps the
//! planet as it was.
//!
//! The save is bincode (serde, no field names): a header (`HEAD`, the
//! build's commit, the caller's tag), the world's state
//! (`World::save_state`: on the GPU, every engine buffer read back) and
//! the `Game` without its world. Only the build that wrote a save reads
//! it, since any change to a saved struct would read as garbage; a save
//! from another build, or for another planet (the tag), is refused, and
//! the window relands the planet as step C did.
//!
//! The world is not in the serde tree: `from_save` takes a world built fresh
//! for the same planet, reads the world's state into it, and hands it to
//! the deserializer through a parking slot (`parked`), the `Game::world`
//! field's serde default.

use super::{Game, World};
use std::any::Any;
use std::cell::RefCell;

/// The save's first words; a new layout of the header bumps the number.
pub const HEAD: &str = "plasmodia planet save 1";

thread_local! {
    static PARKED: RefCell<Option<Box<dyn Any>>> = const { RefCell::new(None) };
}

/// The world `from_save` parked, for `Game::world`'s serde default. Only
/// `Game::from_save` deserializes a `Game`.
pub(super) fn parked<W: 'static>() -> W {
    PARKED.with(|p| p.borrow_mut().take()).and_then(|b| b.downcast::<W>().ok()).map(|b| *b).expect("a Game is only read by Game::from_save")
}

fn put<T: serde::Serialize + ?Sized>(out: &mut Vec<u8>, v: &T) -> Result<(), String> {
    bincode::serialize_into(out, v).map_err(|e| format!("save: {e}"))
}

fn get<T: serde::de::DeserializeOwned>(r: &mut &[u8]) -> Result<T, String> {
    bincode::deserialize_from(r).map_err(|e| format!("load: {e}"))
}

impl<W: World + 'static> Game<W> {
    /// The whole game, between steps: `tag` names the planet (the window's
    /// planet and run), checked by `from_save`.
    pub fn save(&mut self, tag: &str) -> Result<Vec<u8>, String> {
        if self.flight.is_some() {
            return Err("save: a step is still running".into());
        }
        // A walking field still building on its thread is waited for: a
        // thread can't be written.
        self.raids.settle_ways();
        self.bosses.settle_ways();
        let mut out = Vec::new();
        put(&mut out, HEAD)?;
        put(&mut out, crate::build::build().commit)?;
        put(&mut out, tag)?;
        put(&mut out, &self.world.save_state()?)?;
        put(&mut out, &*self)?;
        Ok(out)
    }

    /// The game `save` wrote, on `world`, built fresh for the same planet.
    /// Refuses a save from another build or with another tag.
    pub fn from_save(bytes: &[u8], tag: &str, mut world: W) -> Result<Self, String> {
        let mut r = bytes;
        let head: String = get(&mut r)?;
        if head != HEAD {
            return Err(format!("load: not a planet save ({head:?})"));
        }
        let commit: String = get(&mut r)?;
        let here = crate::build::build().commit;
        if commit != here {
            return Err(format!("load: saved by build {commit}, this is {here}"));
        }
        let saved: String = get(&mut r)?;
        if saved != tag {
            return Err(format!("load: the save is of {saved:?}, not {tag:?}"));
        }
        let state: Vec<u8> = get(&mut r)?;
        world.load_state(&state)?;
        PARKED.with(|p| *p.borrow_mut() = Some(Box::new(world)));
        let g: Result<Game<W>, String> = get(&mut r);
        // A read that failed before the world field leaves it parked.
        PARKED.with(|p| p.borrow_mut().take());
        let g = g?;
        if !r.is_empty() {
            return Err(format!("load: {} bytes left over", r.len()));
        }
        Ok(g)
    }
}
