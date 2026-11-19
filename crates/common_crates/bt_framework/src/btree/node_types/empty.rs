use crate::btree::outcomes::*;
use crate::btree::node_traits::*;

/// A BTEmpty is an action node that represents doing nothing. 
/// 
/// Useful to terminate branches that have no action available.
#[derive(Debug)]
pub struct BTEmpty {}

impl<X: Clone + Copy + PartialEq> BTAct<X> for BTEmpty {
    fn outcome(&self) -> BTOutcome<X> {
        return BTOutcome::EvaluatedAction(None);
    }
}

impl BTEmpty {
    pub fn new() -> Self {
        BTEmpty {}
    }
}
