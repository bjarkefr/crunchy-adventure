#[derive(Copy, Clone)]
struct DirectionSet(u32);

impl DirectionSet {
	pub fn new() -> DirectionSet {
		DirectionSet(0)
	}

	pub fn set_left(self) -> DirectionSet {
		DirectionSet(self.0 | 1)
	}

	pub fn left(self) -> bool {
		self.0 & 1 != 0
	}

	pub fn set_right(self) -> DirectionSet {
		DirectionSet(self.0 | 2)
	}

	pub fn set_up(self) -> DirectionSet {
		DirectionSet(self.0 | 4)
	}

	pub fn set_down(self) -> DirectionSet {
		DirectionSet(self.0 | 8)
	}

	pub fn to_char(self) -> char {
		match self.0 {
			0 => ' ',
			1 => '←',
			2 => '→',
			3 => '─',
			4 => '↑',
			5 => '┘',
			6 => '└',
			7 => '┴',
			8 => '↓',
			9 => '┐',
			10 => '┌',
			11 => '┬',
			12 => '│',
			13 => '┤',
			14 => '├',
			15 => '┼',
			_ => '?'
		}
	}
}

#[derive(Copy, Clone)]
enum Tile {
	Unassigned,
	Tunnel(DirectionSet),
	Room(u32)
}

impl Tile {
	pub fn to_char(self) -> char {
		match self {
			Tile::Unassigned => '?',
			Tile::Tunnel(dirs) => dirs.to_char(),
			Tile::Room(num) => match num {
				0...9 => match num.to_string().chars().next() {
					Some(n) => n,
					_ => 'E'
				},
				_ => 'X'
			}
		}
	}
}

const LABYRINTH_HEIGHT: usize = 60;
const LABYRINTH_WIDTH: usize = 20;
struct Labyrinth([[Tile; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH]);

impl Labyrinth {
	pub fn to_string(&self) -> String {
		self.0.iter().map(|row| {
			let row_iter = row.iter().map(|tile| tile.to_char());
			row_iter.collect()
			//row_iter.chain(['\n'].iter())
		}).collect()

/*		let a = self.0;
		let b = a[0];
		let c = b.iter();
		let d = c.map(|tile| tile.to_char()).collect();
		d*/
		//self.0[0].iter().map(|tile| { tile.to_char() }).to_string()
	}
}

fn main() {
	//let mut labyrinth: [[Tile; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH] = [[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH];

	//labyrinth[0][0] = Tile::Room(1);

	let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	let mut labyrinth: Labyrinth = Labyrinth([[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH]);

	println!("{}{} - {}", h.to_char(), labyrinth.0[0][0].to_char(), labyrinth.to_string());
}
