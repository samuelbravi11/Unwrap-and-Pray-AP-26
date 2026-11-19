use std::marker::PhantomData;

use crate::btree::node_traits::*;
use crate::btree::outcomes::*;
use crate::status::Status;

/// A BTCondition is a flow node that represents a conditional branch in the behaviour tree. 
/// 
/// It evaluates the condition and evaluates one of the two branches based on the outcome.
#[derive(Debug)]
pub struct BTCondition<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    F: Fn(&Status<T, U>) -> BTOutcome<X>,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> {
    success_branch: S,
    failure_branch: Z,
    condition: F,
    _marker: PhantomData<(&'a (), T, U, X)>,
}

impl<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    F: Fn(&Status<T, U>) -> BTOutcome<X>,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> BTFlow<'a, T, U, X> for BTCondition<'a, T, U, X, F, S, Z>
{
    fn flow(&self, status: &Status<T, U>, outcome: BTOutcome<X>) -> BTOutcome<X> {
        match outcome {
            BTOutcome::EvaluatedAction(_) => {
                unreachable!("Recieved a Result outcome in a BTCondition, this should never happen")
            }
            BTOutcome::Success => self.success_branch.evaluate(status),
            BTOutcome::Failure => self.failure_branch.evaluate(status),
        }
    }

    fn outcome(&self, status: &Status<T, U>) -> BTOutcome<X> {
        (self.condition)(status)
    }
}

impl<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    F: Fn(&Status<T, U>) -> BTOutcome<X>,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> BTCondition<'a, T, U, X, F, S, Z>
{
    pub fn new(success_branch: S, failure_branch: Z, condition: F) -> Self {
        BTCondition {
            _marker: PhantomData,
            success_branch,
            failure_branch,
            condition,
        }
    }
}
