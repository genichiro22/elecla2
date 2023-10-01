fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Game {
    status: Status,
    board: Board,
}

#[derive(Debug, Clone)]
struct Board {
    turn: usize,
    player: [Player; 2],
}

#[derive(Debug, Clone)]
struct Player {
    user_id: usize,
    life: usize,
    hand: Domain,
    library: Domain,
    graveyard: Domain,
    battlefield: Domain,
    waiting: Domain,
}

#[derive(Debug, Clone)]
struct Domain {
    name: String,
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    oracle: Oracle,
}

#[derive(Debug, Clone)]
struct Oracle {
    id: usize,
    name: String,
    cost: usize,
}

#[derive(Debug, Clone)]
struct Status {
    id: usize,
    turn: usize,
    active_player: Player,
    next_player: Player,
    phase: Phase,
}

#[derive(Debug, Clone)]
struct Phase {
    name: String,
    step_belonged: Step,
}

#[derive(Debug, Clone)]
struct Step {
    name: String,
}
