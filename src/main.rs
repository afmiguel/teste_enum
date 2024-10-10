/// Represents the possible states of an animal.
enum AnimalState {
    Extant,             /// The animal is extant (still exists).
    Extinct,            /// The animal is extinct (no longer exists).
    PossiblyExtinct,    /// The animal is possibly extinct (its status is unclear, but it may be extinct).
}

/// Prints the state of the given animal.
///
/// # Arguments
///
/// * `state` - The current state of the animal, which can be `Extant`, `Extinct`, or `PossiblyExtinct`.
fn print_state(state: AnimalState) {
}

fn main() {
    let state1 = AnimalState::Extant;
    let state2 = AnimalState::Extinct;
    let state3 = AnimalState::PossiblyExtinct;

    // Prints the state of each animal
    print_state(state1);
    print_state(state2);
    print_state(state3);
}

