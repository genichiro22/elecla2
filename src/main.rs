fn main() {
    println!("Hello, world!");
    let mut player1 = Player {
        life: 20,
        hand: Domain::new("HAND"),
        library: Domain::new("LIBRARY"),
        battlefield: Domain::new("BATTLEFIELD")
    };
    let mut player2 = Player {
        life: 30,
        hand: Domain::new("HAND"),
        library: Domain::new("LIBRARY"),
        battlefield: Domain::new("BATTLEFIELD")
    };
    let mut game = Game {
        status: Status {
            turn: 0,
            active_player: player1.clone(),
            next_player: player2.clone(),
            phase: Phase {
                name: "BOOT",
                step_belonged: Step {
                    name: "BOOT"
                }
            }
        },
        board: Board {
            turn: 0,
            player: [player1.clone(), player2]
        }
    };
    println!("{:?}",game.board.player[0].clone());
    player1.life = 21;
    println!("{:?}",game.board.player[0].clone());
    player1.life = 21;
    
}

#[derive(Debug)]
struct Game<'a> {
    status: Status<'a>,
    board: Board<'a>,
}

#[derive(Debug, Clone)]
struct Board<'a> {
    turn: usize,
    player: [Player<'a>; 2],
}

#[derive(Debug, Clone)]
struct Player<'a> {
    life: usize,
    hand: Domain<'a>,
    library: Domain<'a>,
    // graveyard: Domain,
    battlefield: Domain<'a>,
    // waiting: Domain,
}

trait Agent {
    fn play_card(&self);
    fn attack(&self);
}

#[derive(Debug, Clone)]
struct Domain<'a> {
    name: &'a str,
    cards: Vec<Card>,
}

impl<'a> Domain<'a> {
    fn new(name: &'a str) -> Self {
        Domain {
            name,
            cards: Vec::new(),
        }
    }
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

#[derive(Debug, Clone)]
struct Status<'a> {
    turn: usize,
    active_player: Player<'a>,
    next_player: Player<'a>,
    phase: Phase<'a>,
}

#[derive(Debug, Clone, Copy)]
struct Phase<'a> {
    name: &'a str,
    step_belonged: Step<'a>,
}

#[derive(Debug, Clone, Copy)]
struct Step<'a> {
    name: &'a str,
}
