use crate::Event;

use std::default::Default;

use serde::{Serialize, de::DeserializeOwned};

/// The `State` trait is used to represent the aggregated state of a domain
/// entity after a sequence of events has been applied to it.
/// 
/// Each `State` will usually have its own `Aggregate` that manages the events
/// that are applied to it.
/// 
/// It requires a number of other traits to be implemented, notably `Serialize`
/// and `DeserializeOwned` to allow optimized reading from a `Store`, as well as
/// `Clone` to allow for introspection from outside an `Aggregate`.
/// 
/// Note that `State` is also explicitly marked as `Send + Sync` due to usages
/// of the `async-trait` crate on the `Store` trait.
/// 
/// Implementations of `State` must specify the type of `Identifier` used as
/// well as the type of `Event` that will be used to aggregate state.
/// 
/// An example implementation of `State` for a fictional "Order" entity is
/// provided below.  This example aligns with the examples given for the other
/// core traits.
/// 
/// ```
/// #[derive(Clone, Default, Serialize, Deserialize)]
/// struct OrderState {
///     id: u128,
///     product_name: String,
///     balance_owing: f64
/// }
/// 
/// impl ljprs_es::State for OrderState {
///     type Identifier = u128;
///     type Event = Event;
/// 
///     fn id(&self) -> Self::Identifier {
///         self.id
///     }
/// 
///     fn logical_version() -> u32 {
///         0
///     }
/// 
///     fn apply(&mut self, event: &Self::Event) {
///         match event {
///             Event::OrderCreated(e) => self.apply_order_created(e),
///             Event::OrderPayment(e) => self.apply_order_payment(e)
///         }
///     }
/// }
/// 
/// impl OrderState {
///     fn apply_order_created(&mut self, event: &OrderCreatedEvent) {
///         self.id = event.id;
///         self.product_name = event.product_name.clone();
///         self.balance_owing = event.balance_owing;
///         self.is_fully_paid = false;
///     }
/// 
///     fn apply_order_payment(&mut self, event: &OrderPaymentEvent) {
///         self.balance_owing -= event.amount;
///     }
/// }
/// ```
pub trait State : Send + Sync + Clone + Default + Serialize + DeserializeOwned {
    /// This associated type will usually be dictated by the implementation of
    /// `Store` that you are using.
    type Identifier : Copy + ToString;

    /// Your own `Event` type - see the documentation for the [`Event`] trait
    /// for more information.
    type Event : Event;

    /// Used to determine the Stream identifier of the `State`.
    fn id(&self) -> Self::Identifier;

    /// Used to version the logic used to aggregate state.  This allows for
    /// optimized storage of `State` in `Store` implementations.  If the logic
    /// inside `apply` changes, the version returned from this method should be
    /// incremented.
    fn logical_version() -> u32;

    /// When events need to be applied to the state, they are done so with this
    /// call.
    fn apply(&mut self, event: &Self::Event);
}
