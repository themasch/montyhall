#[derive(Debug)]
struct Doors {
    open: Vec<bool>,
    winner: usize,
    open_count: usize,
    player_pick: usize,
    size: usize
}

impl Doors {
    fn create(size: usize, initial_pick: usize) -> Doors {
        let winner = rand::random::<usize>() % size;
        let open = (0..size).map(|_| true).collect();

        Doors { size, winner, open, open_count: size, player_pick: initial_pick }
    }
}

trait Player { 
    fn change_pick(&self, doors: &Doors) -> Option<usize>;
}

#[derive(Debug)]
struct AlwaysStay;

/// pick once, stay with it. 
impl Player for AlwaysStay {
    fn change_pick(&self, _doors: &Doors) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
struct ChangeLastRound;

/// a player that is smart and changes to the last available door in the last round
impl Player for ChangeLastRound {
    fn change_pick(&self, doors: &Doors) -> Option<usize> {
        if doors.open_count == 2 {
            doors.open.iter().enumerate().filter( | (_, &open) | open ).filter(| (id, _) | *id != doors.player_pick ).map(| (id, _) | id).nth(0)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ChangeAllTheTime;

/// impatient player that takes every given chance to change his/her mind and changes the pick
impl Player for ChangeAllTheTime {
    fn change_pick(&self, doors: &Doors) -> Option<usize> {
        let available_ids = doors.open.iter().enumerate().filter( | (_, &open) | open ).filter(| (id, _) | *id != doors.player_pick ).map(| (id, _) | id).collect::<Vec<usize>>();
        let rng = rand::random::<usize>() % available_ids.len();

        Some(available_ids[rng])
    }
}

#[derive(Debug)]
struct Game<T> where T: Player {
    player: T,
    doors: Doors
}

impl<T: Player> Game<T> {

    fn create(size: usize, player: T) -> Game<T> {
       let initial_pick = rand::random::<usize>() % size;
        Game { player, doors: Doors::create(size, initial_pick) }
    }

    fn turn(&mut self) -> bool {
        // lets find a list of doors we could show to the player (to show him a goat)
        let open_door_ids = self.doors.open.iter()
            .enumerate()
            .filter(| (idx, &open) | 
                    // we will only show (or close) open doors..
                    open && 
                    // that are not the current player pick (that would be stupid)...
                    *idx != self.doors.player_pick &&
                    // and no the winner (that would be even more stupid)
                    *idx != self.doors.winner
            )
            .map(| (idx, _) | idx )
            .collect::<Vec<_>>();
    
        // close a random one of them
        let rand: usize = rand::random();
        let close_door = rand % open_door_ids.len();
        self.doors.open[open_door_ids[close_door]] = false;
        self.doors.open_count -= 1;

        // ask the player if he wants to pick a different door
        match self.player.change_pick(&self.doors) {
            None => {},
            Some(new_id) => {
                self.doors.player_pick = new_id
            }
        };

        self.doors.open_count > 2
    }
     
    /// player picked to correct door?
    fn has_won(&self) -> bool {
        debug_assert!(self.doors.open[self.doors.winner]);
        self.doors.player_pick == self.doors.winner
    }
} 


/// simulate a fulle game with {size} doors for a given player and return if the player won
fn run_test<T: Player>(size: usize, player: T) -> bool {
    let mut game = Game::create(size, player);

    while game.turn() {};

    game.has_won()
}

fn main() {
    let n = 100000;
    let gs = 10;
    let win_rate = (0..n).map(|_| run_test(gs, AlwaysStay)).filter(| &won | won).count();
    println!("==> AlwaysStay      {:>3}", win_rate as f32 / n as f32 * 100.0);
    let win_rate = (0..n).map(|_| run_test(gs, ChangeLastRound)).filter(| &won | won).count();
    println!("==> ChangeLastRound {:>3}", win_rate as f32 / n as f32 * 100.0);
}
