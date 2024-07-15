pub mod utils;
pub mod router;
pub mod internal;

include!(concat!(env!("OUT_DIR"), "/commands_mod.rs"));

pub use internal::*;
pub use utils::*;
pub use router::{Command, CommandRouter};
