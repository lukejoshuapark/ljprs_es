use serde::{de::DeserializeOwned, Serialize};

/// The `Event` is the core trait in this crate.
/// 
/// Each `State` that you create is allowed to specify the type of `Event` it
/// will aggregate.  Notably the `State` only allows a single type to be
/// specified, so for this reason, it is recommended that implementations of
/// `Event` are enums.
/// 
/// It requires a number of other traits to be implemented, notably `Serialize`
/// and `DeserializeOwned` to allow serialization to and from a `Store`.
/// 
/// Note that `Event` is also explicitly marked as `Send + Sync` due to usages
/// of the `async-trait` crate on the `Store` trait.
/// 
/// An example implementation of `Event` for some fictional "Order" related
/// events is provided below.  This example aligns with the examples given for
/// the other core traits.
/// 
/// ```
/// #[derive(AsRefStr, Serialize, Deserialize)]
/// enum Event {
///     OrderCreated(OrderCreatedEvent),
///     OrderPayment(OrderPaymentEvent),
///     OrderPaidOff(OrderPaidOffEvent)
/// }
/// 
/// impl ljprs_es::Event for Event {
///     fn type_name(&self) -> &str {
///         self.as_ref()
///     }
/// }
/// 
/// #[derive(Serialize, Deserialize)]
/// struct OrderCreatedEvent {
///     id: u128,
///     product_name: String,
///     balance_owing: f64
/// }
/// 
/// #[derive(Serialize, Deserialize)]
/// struct OrderPaymentEvent {
///     id: u128,
///     amount: f64
/// }
/// 
/// #[derive(Serialize, Deserialize)]
/// struct OrderPaidOffEvent {
///     id: u128
/// }
/// ```
/// 
/// The `AsRefStr` derive comes from the `strum_macros` crate and is an easy way
/// to satisfy the `type_name` method when implementing `Event` for an enum.
pub trait Event : Send + Sync + Serialize + DeserializeOwned {
    /// The `type_name` method is used by implementations of `Store` to record
    /// the associated name of an event for later deserialization.
    fn type_name(&self) -> &str;
}
