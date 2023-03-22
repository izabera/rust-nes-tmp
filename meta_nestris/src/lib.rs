//! ```no_run
//! use meta_nestris::{Movie, State};
//!
//! let movie = Movie::from_fm2(&"inputs.fm2".into()).expect("File not found.");
//!
//! let mut state = State::new();
//! for input in movie.inputs {
//!     state.step(input);
//! }
//!
//! match state {
//!     State::GameplayState(gameplay_state) => {
//!         println!("gameplay - score: {}", gameplay_state.score);
//!     }
//!     State::MenuState(menu_state) => {
//!         println!("menu: {}", menu_state.menu_mode);
//!     }
//! }
//! ```

#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params)]

mod game_mode_state;
mod game_type;
mod gameplay_state;
mod input;
mod menu_mode;
mod menu_state;
mod modifier;
mod piece;
mod play_state;
mod random;
mod state;

pub use game_mode_state::*;
pub use game_type::*;
pub use gameplay_state::*;
pub use input::*;
pub use menu_mode::*;
pub use menu_state::*;
pub use modifier::*;
pub use piece::*;
pub use play_state::*;
pub use random::*;
pub use state::*;
