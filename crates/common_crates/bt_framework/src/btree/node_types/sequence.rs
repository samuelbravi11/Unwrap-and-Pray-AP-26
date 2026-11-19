use crate::btree::node_traits::*;
use crate::btree::outcomes::*;
use crate::status::Status;
use std::marker::PhantomData;

/// A BTSequence is a flow node that represents a sequence of actions to be executed in order. It evaluates the first branch, then the second one. 
/// 
/// The outcome of the sequence is the combination of the outcomes of both branches.
#[derive(Debug)]
pub struct BTSequence<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> {
    now_branch: S,
    then_branch: Z,
    _marker: PhantomData<(&'a (), T, U, X)>,
}

impl<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> BTFlow<'a, T, U, X> for BTSequence<'a, T, U, X, S, Z>
{
    fn flow(&self, status: &Status<T, U>, outcome: BTOutcome<X>) -> BTOutcome<X> {
        match outcome {
            BTOutcome::Success => {
                let now_outcome = self.now_branch.evaluate(status);
                let then_outcome = self.then_branch.evaluate(status);
                match (now_outcome, then_outcome) {
                    (BTOutcome::EvaluatedAction(now), BTOutcome::EvaluatedAction(then)) => {
                        return BTOutcome::EvaluatedAction(match (now, then) {
                            (Some(now), Some(then)) => {
                                let mut combined = now;
                                combined.extend(then);
                                Some(combined)
                            }
                            (Some(now), None) => Some(now),
                            (None, Some(then)) => Some(then),
                            (None, None) => None,
                        });
                    }
                    _ => {
                        unreachable!(
                            "Recieved a Result outcome in a BTSequence, this should never happen"
                        )
                    }
                }
            }
            _ => {
                unreachable!("Recieved a Result outcome in a BTCondition, this should never happen")
            }
        }
    }

    fn outcome(&self, _status: &Status<T, U>) -> BTOutcome<X> {
        BTOutcome::Success
    }
}

impl<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
    S: BTNode<'a, T, U, X>,
    Z: BTNode<'a, T, U, X>,
> BTSequence<'a, T, U, X, S, Z>
{
    pub fn new(now_branch: S, then_branch: Z) -> Self {
        BTSequence {
            now_branch,
            then_branch,
            _marker: PhantomData,
        }
    }
}
