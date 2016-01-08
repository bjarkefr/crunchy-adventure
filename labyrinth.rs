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

fn main() {
	let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	println!("{}", h.to_char());
}
