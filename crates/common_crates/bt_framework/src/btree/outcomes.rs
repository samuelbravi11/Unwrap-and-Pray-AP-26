/// A possible outcome of a behaviour tree node execution. It can be either a success, a failure or a vector of evaluated actions. The evaluated actions are used to communicate the actions that the node has decided to take, and can be used to trigger the execution of those actions in the environment.
/// It's advised to use an enum to represent the actions, and to derive the Default, Clone, Copy and PartialEq traits for that enum.
#[derive(Clone, PartialEq, Debug)]
pub enum BTOutcome<X> {
    /// Represents a successful outcome of a node execution. Mainly used in flow nodes to represent a successful check.
    Success,
    /// Represents a failed outcome of a node execution. Mainly used in flow nodes to represent a failed check.
    Failure,
    /// Represents a vector of actions that the node has decided to take. Used in action nodes to communicate the actions that the BT has decided to take.
    EvaluatedAction(Option<Vec<X>>),
}
