//! World status ( inputs for the BT ) definition

/// A structure that contains the current status of the world, divided in volatile and hard states.
///
/// ---
///
/// The status **HAS TO BE OWNED OUTSIDE OF THE BRAIN STRUCT**
///
/// ---
///
/// **Altough any structure will work, it's advised to use a structure containing only u32 fields.**
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Status<T: Clone + Copy + PartialEq + Default, U: Clone + Copy + PartialEq + Default> {
    /// Volatile states are values that change frequently and should not trigger a behaviour re-evaluation
    volatile_states: T,

    /// Hard states trigger a behaviour re-evaluation when changed
    hard_states: U,
}

impl<T: Clone + Copy + PartialEq + Default, U: Clone + Copy + PartialEq + Default> Status<T, U> {
    pub fn new(volatile_states: T, hard_states: U) -> Self {
        Status {
            volatile_states,
            hard_states,
        }
    }

    /// Creates a new Status with default values for the hard states and the provided volatile states.
    pub fn from_volatile_states(volatile_states: T) -> Self {
        Status {
            volatile_states,
            hard_states: U::default(),
        }
    }

    /// Creates a new Status with default values for the volatile states and the provided hard states.
    pub fn from_hard_states(hard_states: U) -> Self {
        Status {
            volatile_states: T::default(),
            hard_states,
        }
    }

    /// Returns a reference to the volatile states
    pub fn getref_volatile_states(&self) -> &T {
        return &self.volatile_states;
    }

    /// Returns a reference to the hard states
    pub fn getref_hard_states(&self) -> &U {
        return &self.hard_states;
    }
    /// Updates the volatile states with the provided new set.
    ///
    /// **PREFER TO USE THE METHOD PROVIDED BY `Brain`**
    pub fn update_volatile_states(&mut self, new_volatile_states: T) {
        self.volatile_states = new_volatile_states;
    }
    /// Updates the hard states with the provided new set.
    ///
    /// **PREFER TO USE THE METHOD PROVIDED BY `Brain`**
    pub fn update_hard_states(&mut self, new_hard_states: U) {
        self.hard_states = new_hard_states;
    }
}
