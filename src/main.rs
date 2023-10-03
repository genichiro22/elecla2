fn main() {
    println!("Hello, world!");
    let mut player1 = Player {
        life: 20,
        hand: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
        library: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
        battlefield: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
    };
    let mut player2 = Player {
        life: 30,
        hand: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
        library: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
        battlefield: Domain {
            name: DomainName::Hand,
            cards: Vec::new(),
        },
    };
    let mut game = Game {
        turn: 0,
        player: [player1, player2],
        active_player: 0,
        next_player: 0,
    };
    println!("{:?}",game.player[0]);
    println!("{:?}",game.player[1]);
}

#[derive(Debug)]
struct Game {
    turn: usize,
    player: [Player; 2],
    // phase: Phase<'a>,
    active_player: usize,
    next_player: usize,
}

#[derive(Debug, Clone)]
struct Player {
    life: usize,
    hand: Domain,
    library: Domain,
    // graveyard: Domain,
    battlefield: Domain,
    // waiting: Domain,
}

trait Agent {
    fn play_card(&self);
    fn attack(&self);
}

#[derive(Debug, Clone)]
struct Domain {
    name: DomainName,
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
enum DomainName {
    Hand,
    Battlefield,
    Library,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    id: usize,
    oracle: Oracle,
}

#[derive(Debug, Clone, Copy)]
struct Oracle {
    id: usize,
    // name: String,
    cost: usize,
}

#[derive(Debug, Clone, Copy)]
struct Phase {
    name: PhaseName,
    step_belonged: Step,
}

#[derive(Debug, Clone, Copy)]
enum PhaseName {
    Draw,
    Action,
    Cleanup,
}

#[derive(Debug, Clone, Copy)]
struct Step {
    name: StepName,
}

#[derive(Debug, Clone, Copy)]
enum StepName {
    Boot,
    Main,
    Pass
}