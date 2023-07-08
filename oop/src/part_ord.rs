#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(i8)]
// Allows us to assign integer values to each variant
// This helps us with ordering
pub enum Status {
    Done = 1,
    InProgress = 0,
    Planned = -1
}

pub fn sort_status(stattuses: &mut Vec<Status>) {
    // to sort we will use sort_by
    // to complare we will use partial_cmp
    // we will order from highest to lowest
    stattuses.sort_by(|low, high|
                      high.partial_cmp(&low).unwrap());
}
