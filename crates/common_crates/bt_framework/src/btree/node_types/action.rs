use crate::btree::node_traits::*;
use crate::btree::outcomes::*;

/// A BTAction is an action node that represents one or more actions to be executed.
#[derive(Debug)]
pub struct BTAction<X: Clone + Copy + PartialEq> {
    behaviour: Vec<X>,
}

impl<X: Clone + Copy + PartialEq> BTAct<X> for BTAction<X> {
    fn outcome(&self) -> BTOutcome<X> {
        return BTOutcome::EvaluatedAction(Some(self.behaviour.clone()));
    }
}

impl<X: Clone + Copy + PartialEq> BTAction<X> {
    pub fn new(behaviour: Vec<X>) -> Self {
        BTAction { behaviour }
    }
}
