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
            name: DomainName::Library,
            cards: Vec::new(),
        },
        battlefield: Domain {
            name: DomainName::Battlefield,
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
            name: DomainName::Library,
            cards: Vec::new(),
        },
        battlefield: Domain {
            name: DomainName::Battlefield,
            cards: Vec::new(),
        },
    };
    let mut game = Game {
        turn: 0,
        player: [player1, player2],
        active_player: 0,
        next_player: 1,
    };
    let oracle1 = Oracle {
        id: 1,
        cost: 3
    };
    let oracle2 = Oracle {
        id: 2,
        cost: 5
    };
    let oracle3 = Oracle {
        id: 3,
        cost: 6
    };
    let card1 = Card {
        id: 1,
        oracle: oracle1
    };
    let card2 = Card {
        id: 2,
        oracle: oracle1
    };
    let card3 = Card {
        id: 3,
        oracle: oracle1
    };
    let card4 = Card {
        id: 4,
        oracle: oracle2
    };
    let card5 = Card {
        id: 5,
        oracle: oracle2
    };
    let card6 = Card {
        id: 6,
        oracle: oracle3
    };
    let card7 = Card {
        id: 7,
        oracle: oracle1
    };
    let card8 = Card {
        id: 8,
        oracle: oracle2
    };
    let card9 = Card {
        id: 9,
        oracle: oracle2
    };
    let card10 = Card {
        id: 10,
        oracle: oracle3
    };
    let deck1 = vec![card1, card2, card3, card4, card5];
    let deck2 = vec![card6, card7, card8, card9, card10];
    game.player[0].library.cards = deck1;
    game.player[1].library.cards = deck2;
    game.debug();
    game.player[0].draw_a_card();
    game.pass_turn();
    game.debug();
    game.pass_turn();
    game.debug();
    game.pass_turn();
}

fn turn_loop(mut game: Game) {
    game.pass_turn();
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
    fn debug(&mut self) {
        println!("{:#?}", self);
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
impl Player {
    fn draw_a_card(&mut self) {
        let length = self.library.cards.len();
        if length>0 {
            let card = self.library.cards.remove(0);
            self.hand.cards.push(card);    
        } else {
            println!("No card in your library.")
        }
    }
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