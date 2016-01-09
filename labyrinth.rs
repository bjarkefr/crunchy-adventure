use std::rand::{thread_rng, Rng};

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

struct Coord {
	x: u32,
	y: u32
}

impl Coord {
	pub fn to_string(self) -> String {
		format!("({}, {})", self.x, self.y)
	}
}

struct Area {
	min: Coord,
	max: Coord
}

impl Area {
	pub fn random_subarea(self) -> Area {
		let rng = thread_rng();
		let min = Coord { x: rng.gen_range(self.min.x, self.max.x + 1), y: rng.gen_range(self.min.y, self.max.y + 1) };
		let max = Coord { x: rng.gen_range(min.x, self.max.x + 1), y: rng.gen_range(min.y, self.max.y + 1) };

		Area { min: min, max: max }
	}

	pub fn to_string(self) -> String {
		format!("[{} - {}]", self.min, self.max)
	}
}

impl Labyrinth {
	/*fn iter(&self, area: Area) -> Box<Iterator<Item=Tile>> { //iter::Map<iter::Take<iter::Skip<slice::Iter<Tile>>>> {
		let rowIter = self.0.iter().skip(area.min.y as usize).take((area.max.y - area.min.y) as usize);

		Box::new(rowIter.map(|row| row.iter().skip(area.min.x as usize).take((area.max.x - area.min.x) as usize)))
	}*/ // @TOBY: HEEEEEEEEEEELP!!!!

	fn area_unassigned(&self, area: Area) -> bool {
		self.0[area.min.y as usize .. area.max.y as usize].iter().all(|row| {
			row[area.min.x as usize .. area.max.x as usize].iter().all(|&tile| match tile {
				Tile::Unassigned => true,
				_ => false
			})
		})
	}

	fn place(&mut self, tile: Tile, area: Area) {
		for y in area.min.y as usize .. area.max.y as usize {
			for x in area.min.x as usize .. area.max.x as usize {
				self.0[y][x] = tile;
			}
		}
	}

	fn place_rooms(&mut self, n: u32) {

	}

	pub fn to_string(&self) -> String {
		self.0.iter().map(|row| {
			let row_iter = row.iter().map(|tile| tile.to_char()).chain("\n".chars());
			
			row_iter.collect::<String>()
		}).collect()
	}
}

fn main() {
	//let mut labyrinth: [[Tile; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH] = [[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH];

	//labyrinth[0][0] = Tile::Room(1);

	let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	let mut labyrinth = Labyrinth([[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH]);

	println!("{}{} - {}", h.to_char(), labyrinth.0[0][0].to_char(), labyrinth.to_string());

	println!("{}", Area { min: Coord { x: 0, y: 9 }, max: Coord { x: 0, y: 9 } }.to_string());
}
