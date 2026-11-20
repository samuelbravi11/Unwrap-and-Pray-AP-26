use crate::btree::node_traits::*;
use crate::btree::outcomes::*;
use crate::status::Status;

/// A BTNull is a utility node used to specify the non-used generic in the BTBranch.
///
/// **DO NOT USE AS AN ACTUAL NODE**
#[derive(Debug)]
pub struct BTNull {}

impl<X: Clone + Copy + PartialEq> BTAct<X> for BTNull {
    fn outcome(&self) -> BTOutcome<X> {
        return BTOutcome::EvaluatedAction(None);
    }
}

impl<'a, T: Clone + Copy + PartialEq + Default, U: Clone + Copy + PartialEq + Default, X: Clone + Copy + PartialEq>
    BTFlow<'a, T, U, X> for BTNull
{
    #[allow(unused_variables)]
    fn flow(&self, status: &Status<T, U>, outcome: BTOutcome<X>) -> BTOutcome<X> {
        BTOutcome::Failure
    }

    fn outcome(&self, _status: &Status<T, U>) -> BTOutcome<X> {
        BTOutcome::Failure
    }
}

impl BTNull {
    pub fn new() -> Self {
        BTNull {}
    }
}
