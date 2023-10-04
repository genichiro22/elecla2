fn main() {
    println!("Hello, world!");
    let mut player1 = Player {
        life: 20,
        local_turn: 0,
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
        local_turn: 0,
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
        next_player: 1,
    };
    println!("{:?}", game.clone());
    game.pass_turn();
    println!("{:?}", game.clone());
    game.pass_turn();
    println!("{:?}", game.clone());
    game.pass_turn();
}

fn turn_loop(mut _game: Game) {
    _game.pass_turn();
}

#[derive(Debug, Clone)]
struct Game {
    turn: usize,
    player: [Player; 2],
    // phase: Phase<'a>,
    active_player: usize,
    next_player: usize,
}

impl Game {
    fn pass_turn(&mut self) {
        self.active_player = 1 - self.active_player;
        self.next_player = 1 - self.next_player;
        self.turn = self.turn + 1;
        self.player[self.active_player].local_turn = self.player[self.active_player].local_turn + 1;
    }
}

#[derive(Debug, Clone)]
struct Player {
    life: usize,
    local_turn: usize,
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

#[derive(Debug, Clone, Copy)]
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