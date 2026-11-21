use std::vec;

use bt_framework::{
    btree::node_traits::BTBranch, btree::node_types::*, btree::outcomes::BTOutcome, generate_brain, status::Status, *,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum A {
    OUT1,
    OUT2,
    OUT3,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct SH {
    pub val_h: u32,
    pub val_h1: u32,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct SV {
    pub val_v: u32,
    pub val_v1: u32,
}

#[test]
fn btcondition_function() {
    let mut brain = Brain::new(
        Status::<SV, SH>::default(),
        BTCondition::new(
            |s: &Status<SV, SH>| {
                if s.getref_hard_states().val_h == 1 {
                    BTOutcome::Success
                } else {
                    BTOutcome::Failure
                }
            },
            BTBranch::new_action(BTAction::new(vec![A::OUT1])),
            BTBranch::new_action(BTAction::new(vec![A::OUT2])),
        ),
    );

    let mut sh = SH { val_h: 1, val_h1: 0 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT1])
    );
    // No changes : expected same output
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT1])
    );

    sh.val_h = 2;
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT2])
    );
}

#[test]
fn sequence_function() {
    let mut brain = Brain::new(
        Status::<SV, SH>::default(),
        BTSequence::new(
            BTBranch::new_action(BTAction::new(vec![A::OUT1])),
            BTBranch::new_action(BTAction::new(vec![A::OUT2])),
        ),
    );

    let sh = SH { val_h: 1, val_h1: 0 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT1, A::OUT2])
    );
}

#[test]
fn correct_tree_traversal() {
    let mut brain = Brain::new(
        Status::<SV, SH>::default(),
        BTCondition::new(
            |s: &Status<SV, SH>| {
                if s.getref_hard_states().val_h == 1 {
                    BTOutcome::Success
                } else {
                    BTOutcome::Failure
                }
            },
            BTBranch::new_flow(BTSequence::new(
                BTBranch::new_action(BTAction::new(vec![A::OUT1])),
                BTBranch::new_flow(BTCondition::new(
                    |s: &Status<SV, SH>| {
                        if s.getref_hard_states().val_h1 == 1 {
                            BTOutcome::Success
                        } else {
                            BTOutcome::Failure
                        }
                    },
                    BTBranch::new_action(BTAction::new(vec![A::OUT2])),
                    BTBranch::new_action(BTEmpty::new()),
                )),
            )),
            BTBranch::new_flow(BTSequence::new(
                BTBranch::new_action(BTAction::new(vec![A::OUT3])),
                BTBranch::new_action(BTAction::new(vec![A::OUT3])),
            )),
        ),
    );

    let mut sh = SH { val_h: 0, val_h1: 0 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT3, A::OUT3])
    );
    sh = SH { val_h: 0, val_h1: 1 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT3, A::OUT3])
    );
    sh = SH { val_h: 1, val_h1: 0 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT1])
    );
    sh = SH { val_h: 1, val_h1: 1 };
    assert_eq!(
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        Some(vec![A::OUT1, A::OUT2])
    );
}

#[test]
#[should_panic]
fn prevent_condition_return_action() {
    let mut brain = Brain::new(
        Status::<SV, SH>::default(),
        BTCondition::new(
            |_s: &Status<SV, SH>| BTOutcome::<A>::EvaluatedAction(None),
            BTBranch::new_action(BTEmpty::new()),
            BTBranch::new_action(BTEmpty::new()),
        ),
    );

    let sh = SH { val_h: 0, val_h1: 0 };
    brain.update_status(Status::<SV, SH>::from_hard_states(sh));
}

#[test]
fn macro_function() {
    use A::*;
    let mut brain_standard = Brain::new(
        Status::<SV, SH>::default(),
        BTCondition::new(
            |s: &Status<SV, SH>| {
                if s.getref_hard_states().val_h == 1 {
                    BTOutcome::Success
                } else {
                    BTOutcome::Failure
                }
            },
            BTBranch::new_flow(BTSequence::new(
                BTBranch::new_action(BTAction::new(vec![A::OUT1])),
                BTBranch::new_flow(BTCondition::new(
                    |s: &Status<SV, SH>| {
                        if s.getref_hard_states().val_h1 == 1 {
                            BTOutcome::Success
                        } else {
                            BTOutcome::Failure
                        }
                    },
                    BTBranch::new_action(BTAction::new(vec![A::OUT2])),
                    BTBranch::new_action(BTEmpty::new()),
                )),
            )),
            BTBranch::new_flow(BTSequence::new(
                BTBranch::new_action(BTAction::new(vec![A::OUT3])),
                BTBranch::new_action(BTAction::new(vec![A::OUT3])),
            )),
        ),
    );

    let first_cond = |s: &Status<SV, SH>| {
        if s.getref_hard_states().val_h == 1 {
            BTOutcome::<A>::Success
        } else {
            BTOutcome::<A>::Failure
        }
    };
    let second_cond = |s: &Status<SV, SH>| {
        if s.getref_hard_states().val_h1 == 1 {
            BTOutcome::<A>::Success
        } else {
            BTOutcome::<A>::Failure
        }
    };
    let mut brain = generate_brain!(
        Status::<SV, SH>::default(),
        IF{
            first_cond,
            SEQ{
                ACT[OUT1],
                IF{
                    second_cond,
                    ACT[OUT2],
                    ACT[]
                }
            },
            SEQ{
                ACT[OUT3],
                ACT[OUT3]
            }
        }
    );

    let mut sh = SH { val_h: 0, val_h1: 0 };
    assert_eq!(
        brain_standard.update_status(Status::<SV, SH>::from_hard_states(sh)),
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        "T1"
    );
    sh = SH { val_h: 0, val_h1: 1 };
    assert_eq!(
        brain_standard.update_status(Status::<SV, SH>::from_hard_states(sh)),
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        "T2"
    );
    sh = SH { val_h: 1, val_h1: 0 };
    assert_eq!(
        brain_standard.update_status(Status::<SV, SH>::from_hard_states(sh)),
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        "T3"
    );
    sh = SH { val_h: 1, val_h1: 1 };
    assert_eq!(
        brain_standard.update_status(Status::<SV, SH>::from_hard_states(sh)),
        brain.update_status(Status::<SV, SH>::from_hard_states(sh)),
        "T4"
    );
}
