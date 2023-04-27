use crate::State;

/// The `Aggregate` trait is used to manage the life-cycle of implementations of
/// `State`.
/// 
/// Each `Aggregate` implementation will usually have its own `State`
/// implementation that it manages.
/// 
/// Note that `Aggregate` is also explicitly marked as `Send + Sync` due to
/// usages of the `async-trait` crate on the `Store` trait.
/// 
/// Implementations of `Aggregate` must specify the type of `Identifier` used as
/// well as the type of `State` that will be managed.
/// 
/// An example implementation of `Aggregate` for a fictional "Order" entity is
/// provided below.  This example aligns with the examples given for the other
/// core traits.
/// 
/// ```
/// #[derive(Error, Debug, PartialEq)]
/// enum OrderAggregateError {
///     #[error("a payment of this amount would leave a negative balance owing")]
///     Overpayment
/// }
/// 
/// struct OrderAggregate {
///     state: OrderState,
///     next_version: u32,
///     pending_events: Vec<Event>
/// }
/// 
/// impl ljprs_es::Aggregate for OrderAggregate {
///     type Identifier = u128;
///     type State = OrderState;
/// 
///     fn from_state(state: Self::State, next_version: u32) -> Self {
///         OrderAggregate {
///             state: state,
///             next_version: next_version,
///             pending_events: Vec::with_capacity(0)
///         }
///     }
/// 
///     fn clone_state(&self) -> Self::State {
///         self.state.clone()
///     }
/// 
///     fn take(self) -> (Self::State, u32, Vec<<Self::State as ljprs_es::State>::Event>) {
///         (self.state, self.next_version, self.pending_events)
///     }
/// }
/// 
/// impl OrderAggregate {
///     fn new(product_name: String, balance_owing: f64) -> Self {
///         let mut state = OrderState::default();
///         let event = Event::OrderCreated(OrderCreatedEvent {
///             id: rand::thread_rng().gen(),
///             product_name: product_name,
///             balance_owing: balance_owing
///         });
/// 
///         state.apply(&event);
/// 
///         OrderAggregate {
///             state: state,
///             next_version: 1,
///             pending_events: vec!(event)
///         }
///     }
/// 
///     fn make_payment(&mut self, amount: f64) -> Result<(), OrderAggregateError> {
///         if self.state.balance_owing - amount < 0f64 {
///             return Err(OrderAggregateError::Overpayment);
///         }
/// 
///         let event_payment = Event::OrderPayment(OrderPaymentEvent {
///             id: self.state.id,
///             amount: amount
///         });
/// 
///         self.state.apply(&event_payment);
///         self.pending_events.push(event_payment);
///         self.next_version += 1;
/// 
///         Ok(())
///     }
/// }
/// ```
pub trait Aggregate : Send + Sync {
    /// This associated type will usually be dictated by the implementation of
    /// `Store` that you are using.
    type Identifier : Copy + ToString;

    /// The associated `State` implementation that will be managed by this
    /// `Aggregate`.  It must share the same `Identifier` type.
    type State : State<Identifier = Self::Identifier>;

    /// The `from_state` method is used to construct this aggregate from an
    /// existing state and a known next event version.  It is primarily used by
    /// `Store` implementations but is also useful for constructing a brand new
    /// `State` managed by an `Aggregate`.
    fn from_state(state: Self::State, next_version: u32) -> Self;

    /// Required to be implemented to allow safe introspection of the state of
    /// an aggregate.
    fn clone_state(&self) -> Self::State;

    /// Exclusively called by `Store` implementations when it is time to persist
    /// any pending events in an `Aggregate` to the backing data repository.
    /// It intentionally ensures the `Aggregate` cannot be used after the call
    /// completes.
    fn take(self) -> (Self::State, u32, Vec<<Self::State as crate::State>::Event>);
}
