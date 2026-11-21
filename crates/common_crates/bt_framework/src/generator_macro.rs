/// A macro to simplify the creation of a Brain and its respective BT
///
/// ---
///
/// Syntax example:
/// ```ignore
/// let mut brain = generate_brain!(
///     Status::<SV, SH>::default(),
///     IF{
///         first_cond,
///         SEQ{
///             ACT[OUT1],
///             IF{
///                 second_cond,
///                 ACT[OUT2],
///                 ACT[]
///             }
///         },
///         SEQ{
///             ACT[OUT3],
///             ACT[OUT3]
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! generate_brain {
    (
        $status:expr, $node_name:ident $node_fields:tt
    ) => {
        Brain::new(
            $status,
            first_node!($node_name $node_fields),
        );
    };
}

/// First node macro is needed to avoid applyng the Branch::new_flow to the first node ot the BT.
#[macro_export]
macro_rules! first_node {
    (IF $body:tt) => {
        btcond_i!($body)
    };

    (SEQ $body:tt) => {
        btseq_i!($body)
    };
}

/// node_diff checks the node type and calls the respective macro
#[macro_export]
macro_rules! node_diff {
    (IF $body:tt) => {
        btcond!($body)
    };

    (SEQ $body:tt) => {
        btseq!($body)
    };

    (ACT $body:tt) => {
        btact!($body)
    };
}

/// Macro to represent a new BTCondition node
#[macro_export]
macro_rules! btcond {
    ({$fn:expr, $true_ident:ident $true_branch:tt, $false_ident:ident $false_branch:tt}) => {
        BTBranch::new_flow(BTCondition::new(
                $fn,
                node_diff!($true_ident $true_branch),
                node_diff!($false_ident $false_branch),
            )
        )
    };
}

/// Macro to represent an INITIAL BTCondition node
#[macro_export]
macro_rules! btcond_i {
    ({$fn:expr, $true_ident:ident $true_branch:tt, $false_ident:ident $false_branch:tt}) => {
        BTCondition::new(
            $fn,
            node_diff!($true_ident $true_branch),
            node_diff!($false_ident $false_branch),
        )
    };
}

/// Macro to represent a new BTSequence node
#[macro_export]
macro_rules! btseq {
    ({$now_ident:ident $now_branch:tt, $then_ident:ident $then_branch:tt}) => {
        BTBranch::new_flow(BTSequence::new(node_diff!($now_ident $now_branch), node_diff!($then_ident $then_branch)))
    };
}

/// Macro to represent an INITIAL BTSequence node
#[macro_export]
macro_rules! btseq_i {
    ({$now_ident:ident $now_branch:tt, $then_ident:ident $then_branch:tt}) => {
        BTSequence::new(node_diff!($now_ident $now_branch), node_diff!($then_ident $then_branch))
    };
}

/// Macro to represent a new BTAction or BTEmpty (if left empty) node
#[macro_export]
macro_rules! btact {
    ([$($actions:ident),*]) => {
        BTBranch::new_action(BTAction::new(vec![$($actions,)*]))
    };
    ([]) => {
        BTBranch::new_action(BTEmpty::new())
    };
}
