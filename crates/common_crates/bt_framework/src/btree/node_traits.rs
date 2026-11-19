use crate::btree::node_types::BTNull;
use crate::btree::outcomes::BTOutcome;
use crate::status::Status;
use std::fmt::Debug;
use std::marker::PhantomData;

/// The generic node trait for behaviour tree nodes. It defines the basic structure and behavior of a node in the tree.
pub trait BTNode<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
>
{
    /// Evaluate the node and return its outcome.
    fn evaluate(&self, status: &Status<T, U>) -> BTOutcome<X>;
}

/// A trait for action nodes in a behaviour tree. Action nodes represent the final behaviours chosen by the tree..
pub trait BTAct<X> {
    /// The outcome in action nodes represents the result of the action performed by the node.
    fn outcome(&self) -> BTOutcome<X>;
}

/// A trait for flow nodes in a behaviour tree. Flow nodes represent the decision-making process of the tree, determining which branch to take based on the outcome of the current node.
pub trait BTFlow<
    'a,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
>
{
    /// The flow function is responsible for determining the next node to evaluate based on the outcome of the current node.
    ///
    /// It determines the branch that is chosen.
    fn flow(&self, status: &Status<T, U>, condition_outcome: BTOutcome<X>) -> BTOutcome<X>;

    /// The outcome in flow nodes represents the result of the condition check, which is used to determine which branch to take.
    fn outcome(&self, status: &Status<T, U>) -> BTOutcome<X>;
}

/// A BTBranch is a general node in a behaviour tree that can either be an action or a flow.
#[derive(Debug)]
pub enum BTBranch<
    'a,
    M: BTAct<X>,
    N: BTFlow<'a, T, U, X>,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
> {
    Action(M, PhantomData<(&'a (), X, T, U)>),
    Flow(N, PhantomData<(&'a (), X, T, U)>),
}

impl<
    'a,
    N: BTFlow<'a, T, U, X>,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
> BTBranch<'a, BTNull, N, T, U, X>
{
    /// Create a new BTBranch with a flow node.
    ///
    /// It's advised to use this function instead of manually creating a branch and node
    pub fn new_flow(node: N) -> Self {
        BTBranch::<'a, BTNull, N, T, U, X>::Flow(node, PhantomData)
    }
}

impl<
    'a,
    M: BTAct<X>,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
> BTBranch<'a, M, BTNull, T, U, X>
{
    /// Create a new BTBranch with an action node.
    ///
    /// It's advised to use this function instead of manually creating a branch and node
    pub fn new_action(node: M) -> Self {
        BTBranch::<'a, M, BTNull, T, U, X>::Action(node, PhantomData)
    }
}

impl<
    'a,
    M: BTAct<X>,
    N: BTFlow<'a, T, U, X>,
    T: Clone + Copy + PartialEq + Default,
    U: Clone + Copy + PartialEq + Default,
    X: Clone + Copy + PartialEq,
> BTNode<'a, T, U, X> for BTBranch<'a, M, N, T, U, X>
{
    /// Evaluate the node and return its outcome.
    fn evaluate(&self, status: &Status<T, U>) -> BTOutcome<X> {
        match self {
            BTBranch::Action(node, _) => {
                return node.outcome();
            }
            BTBranch::Flow(node, _) => {
                let outcome = node.outcome(status);
                return node.flow(status, outcome);
            }
        }
    }
}
