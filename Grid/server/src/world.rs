pub struct Player {
    id: u32
}

impl Player {
    pub fn new(id: u32) -> Player {
        Player {
            id: id
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
	Wall,
	Floor
}

impl Tile {
	pub fn render(&self) -> char {
		match *self {
			Tile::Wall => '▓',
			Tile::Floor => ' '
		}
	}
}

const LABYRINTH_WIDTH: usize = 40;
const LABYRINTH_HEIGHT: usize = 40;

pub struct World {
    players: Vec<Player>,
    last_player_id: u32,

    map: ([[Tile; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT]),
}

impl World {
    pub fn new() -> World {
        let mut map = [[Tile::Wall; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT];
        map[1][1] = Tile::Floor;

        World {
            players: Vec::new(),
            last_player_id: 0,
            map: map
        }
    }

    pub fn new_player(&mut self) -> &Player {
        self.last_player_id += 1;
        let player = Player::new(self.last_player_id);
        self.players.push(player);

        self.players.last().unwrap()
    }

    pub fn render_for_player(&self) -> String { //, player : &Player
        let mut output = String::new();
        
        for x in 0..10 {
            for y in 0..10 {
                output.push(self.map[x][y].render())
            }
            output.push('\n');
        }

        output
    }
}
