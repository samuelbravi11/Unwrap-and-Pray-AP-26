//! # BT_Framework
//!
//! Framework for creating behaviour trees, mainly aimed at medium sized NPC AIs for games.

pub mod btree;
pub mod status;

use std::marker::PhantomData;

use btree::node_traits::*;
use btree::outcomes::BTOutcome::EvaluatedAction;
use status::Status;

//######## GENERICS GUIDE
// 'a : Status lifetime
// M : ActNode
// M : FlowNode
// T : Volatile States
// U : Hard States
// X : OutcomeType
// S : First Child Node
// Z : Second Child Node
// F : Passed function

//######### TODO LIST
// - Documentation
// - Clean up implementation
// - Helper functions (create branch+node)
// - Possible macro for tree creation

/// A Brain is the main interface for interacting with a bt.
///
/// It holds a reference to the status, the last evaluated behaviour, and the root node of the behaviour tree.
///
/// The Brain is responsible for re-evaluating the behaviour tree based on changes in the status.
pub struct Brain<
    'a,
    T: Default + Clone + Copy + PartialEq,
    U: Default + Clone + Copy + PartialEq,
    X: Clone + Copy + PartialEq,
    S: BTFlow<'a, T, U, X>,
> {
    _marker: PhantomData<&'a ()>,
    /// A reference to the status object that holds the current state of the system. This reference is used to evaluate conditions and make decisions within the behaviour tree.
    status: Status<T, U>,
    /// The last evaluated behaviour, represented as an optional vector of actions. This is used to store the result of the last evaluation of the behaviour tree, so that it can be returned if the hard state has not changed since the last evaluation.
    last_eval_behaviour: Option<Vec<X>>,
    /// The root node of the behaviour tree.
    behaviour_tree: S,
}

impl<
    'a,
    T: Default + Clone + Copy + PartialEq,
    U: Default + Clone + Copy + PartialEq,
    X: Clone + Copy + PartialEq,
    S: BTFlow<'a, T, U, X>,
> Brain<'a, T, U, X, S>
{
    /// Creates a new Brain with the given status and behaviour tree.
    ///
    pub fn new(status: Status<T, U>, tree: S) -> Self {
        let outcome = tree.outcome(&status);
        let new_behaviour = tree.flow(&status, outcome);
        match new_behaviour {
            EvaluatedAction(actions) => Brain {
                _marker: PhantomData,
                status: status,
                last_eval_behaviour: actions,
                behaviour_tree: tree,
            },
            _ => {
                unreachable!("Recieved a Result outcome in a BTFlow")
            }
        }
    }

    /// Returns a reference to the volatile states
    pub fn getref_volatile_states(&self) -> &T {
        return self.status.getref_volatile_states();
    }
    /// Returns a reference to the hard states
    pub fn getref_hard_states(&self) -> &U {
        return &self.status.getref_hard_states();
    }
    /// Updates the volatile states with the provided new set and returns the last evaluated behaviour.
    pub fn update_volatile_states(&mut self, new_volatile_states: T) -> Option<Vec<X>> {
        self.status.update_volatile_states(new_volatile_states);
        return self.last_eval_behaviour.clone();
    }
    /// Updates the hard states with the provided new set and returns the **new** evaluated behaviour.
    pub fn update_hard_states(&mut self, new_hard_states: U) -> Option<Vec<X>> {
        //I wanted to avoid an useless clone (comparing a ref to an actual var). I think it's faster this way (not sure)
        let tmpcmp = &new_hard_states;
        if tmpcmp != self.status.getref_hard_states() {
            self.status.update_hard_states(new_hard_states);
        }
        self.evaluate_bt()
    }
    /// Updates the behaviour tree based on the current status.
    ///
    /// - If the hard state version has changed since the last evaluation, it re-evaluates the behaviour tree and updates the last evaluated behaviour.
    /// - If the hard state version has not changed, it returns the last evaluated behaviour without re-evaluating the tree.
    pub fn update_status(&mut self, status: Status<T, U>) -> Option<Vec<X>> {
        if self.status.getref_hard_states() != status.getref_hard_states() {
            self.status = status;
            return self.evaluate_bt();
        }
        return self.last_eval_behaviour.clone();
    }

    /// Evaluates the behaviour tree and returns the resulting actions.
    fn evaluate_bt(&mut self) -> Option<Vec<X>> {
        let outcome = self.behaviour_tree.outcome(&self.status);
        let new_behaviour = self.behaviour_tree.flow(&self.status, outcome);
        match new_behaviour {
            EvaluatedAction(actions) => {
                self.last_eval_behaviour = actions.clone();
                return actions;
            }
            _ => {
                unreachable!("Recieved a Result outcome in a BTFlow")
            }
        }
    }

    /// Returns the last evaluated behaviour.
    pub fn get_last_eval_behaviour(&self) -> Option<Vec<X>> {
        return self.last_eval_behaviour.clone();
    }
}
