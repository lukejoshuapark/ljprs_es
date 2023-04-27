//! # ljprs_es
//!
//! Provides the core types to facilitate event sourcing in Rust.
//! 
//! ## Logical Versions
//! When event streams get large, there is considerable overhead in rebuilding
//! the current state of an aggregate from all historical events.  There are
//! multiple approaches to reduce the impact that this can have, but this crate
//! provides functionality to allow for state persisting.
//! 
//! It does this by ensuring implementations of `State` can be serialized, and
//! also ensures that implementations of `State` expose a "logical version".
//! 
//! This `u32` value is used to version the logic used to produce the `State`
//! from a stream of events.
//! 
//! This allows implementations of `Store` to persist the current state of an
//! aggregate to the backing data repository.  When reading the stream, only the
//! `State` needs to be read if we know the logic used to produce that state has
//! not changed.
//! 
//! The full event stream will only need to be read and aggregated if the logic
//! used to produce the state has changed.  For this reason, it is important to
//! ensure the logical version of a `State` is incremented if the logic used to
//! produce it changes.

mod aggregate;
mod event;
mod state;
mod store;

pub use aggregate::Aggregate;
pub use event::Event;
pub use state::State;
pub use store::Store;
