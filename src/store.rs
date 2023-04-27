use crate::{Aggregate};

use std::error::Error;

use async_trait::async_trait;

/// The `Store` trait is implemented by types that are capable of persisting
/// events to a backing data repository.
/// 
/// This crate does not directly provide any implementations of `Store` but
/// rather encourages implementations to be provided in external crates.
/// 
/// It uses the `async-trait` crate to allow async implementations.
#[async_trait]
pub trait Store {
    /// The type of identifier used by the backing data repository.  Note the
    /// usage of this associated type in the `get` method - it is the type by
    /// which existing streams of events are retrieved.
    type Identifier : Copy;

    /// The type of error returned when a call to `get` or `save` fails.
    type Error : Error;

    /// Should retrieve the state and/or events for the stream with the provided
    /// identifier and construct an aggregate around an up-to-date state.
    async fn get<A : Aggregate<Identifier = Self::Identifier>>(&self, id: Self::Identifier) -> Result<A, Self::Error>;

    /// Should persist any pending events in the aggregate to the backing data
    /// repository, as well as the current state if possible.
    async fn save<A : Aggregate<Identifier = Self::Identifier>>(&self, aggregate: A) -> Result<(), Self::Error>;
}
