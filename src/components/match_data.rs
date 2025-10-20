#[derive(Debug, Default, Clone)]
pub struct MatchData {
    //try my best not always operate on these values directly
    pub round_count: usize,
    //represents the players
    pub turn: Option<usize>,
}

impl MatchData {
    pub fn new() -> Self {
        MatchData {
            round_count: 0,
            turn: None,
        }
    }
}
