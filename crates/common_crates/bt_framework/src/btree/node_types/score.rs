use crate::nodes::node_outcomes::*;
use crate::nodes::node_traits::*;
use crate::status::Status;
use std::marker::PhantomData;

//WIP

#[derive(Debug)]
pub struct BTSequence<
    'a,
    T: Clone + Copy + PartialEq + Default,
    N: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq + Default,
    S: BTNode<X>,
    U: BTNode<X>,
> {
    status: &'a Status<T, N>,
    now_branch: S,
    then_branch: U,
    _marker: PhantomData<X>,
}

impl<
    'a,
    T: Clone + Copy + PartialEq + Default,
    N: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq + Default,
    S: BTNode<X>,
    U: BTNode<X>,
> BTFlow<X> for BTSequence<'a, T, N, X, S, U>
{
    fn flow(&self, outcome: BTOutcome<X>) -> BTOutcome<X> {
        match outcome {
            BTOutcome::Success => {
                let now_outcome = self.now_branch.evaluate();
                let then_outcome = self.then_branch.evaluate();
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

    fn outcome(&self) -> BTOutcome<X> {
        BTOutcome::Success
    }
}
