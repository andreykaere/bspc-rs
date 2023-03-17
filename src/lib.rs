pub mod communication;
pub mod errors;
pub mod events;
mod parser;
pub mod properties;
pub mod query;
// pub mod selectors;
// pub mod settings;
mod bspc;
pub mod tree;

pub type Id = u32;

#[doc(inline)]
pub use events::subscribe;

#[doc(inline)]
pub use query::{query_desktops, query_monitors, query_nodes, query_tree};
