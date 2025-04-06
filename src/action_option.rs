#[derive(Debug, Clone, Copy)]
/// The ActionOption enum is the set of possible actions that can be performed by a user
/// The intended way to use this enum is to take a subset of the enum
/// (a vector of specific variants), and pass it to an implementation of the Input trait
/// to ask the user to pick one of the actions to perform, after which it will be converted
/// to its corresponding Action variant (Action enum)
pub enum ActionOption {
    Ante,
    Call,
    Bet,
    Raise,
    Check,
    AllIn,
    Fold,
    Replace,
    Win,
    Lose,
}


