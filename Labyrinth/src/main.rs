#![allow(dead_code)]

extern crate rand;
use rand::Rng;
use std::cmp;

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

struct Vector {
	x: u32,
	y: u32
}

impl Vector {
	pub fn new(x: u32, y: u32) -> Vector {
		Vector { x: x, y: y }
	}

	pub fn to_string(&self) -> String {
		format!("({}, {})", self.x, self.y)
	}
}

struct Area {
	min: Vector,
	max: Vector
}

impl Area {
	pub fn new(min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> Area {
		Area { min: Vector { x: min_x, y: min_y }, max: Vector { x: max_x, y: max_y } }
	}

	pub fn dimensions(&self) -> Vector {
		Vector::new(self.max.x - self.min.x, self.max.y - self.min.y)
	}

	pub fn random_subarea(&self, rng: &mut rand::ThreadRng, min_size: &Vector, max_size: &Vector) -> Area {
		let min = Vector { x: rng.gen_range(self.min.x, self.max.x - min_size.x + 1), y: rng.gen_range(self.min.y, self.max.y - min_size.y + 1) };
		let max = Vector { x: rng.gen_range(min.x + min_size.x, cmp::min(self.max.x, min.x + max_size.x) + 1), y: rng.gen_range(min.y + min_size.y, cmp::min(self.max.y, min.y + max_size.y) + 1) };

		Area { min: min, max: max }
	}

	pub fn to_string(&self) -> String {
		format!("[{} - {}]", self.min.to_string(), self.max.to_string())
	}
}

impl Labyrinth {
	/*fn iter(&self, area: Area) -> Box<Iterator<Item=Tile>> { //iter::Map<iter::Take<iter::Skip<slice::Iter<Tile>>>> {
		let rowIter = self.0.iter().skip(area.min.y as usize).take((area.max.y - area.min.y) as usize);

		Box::new(rowIter.map(|row| row.iter().skip(area.min.x as usize).take((area.max.x - area.min.x) as usize)))
	}*/ // @TOBY: HEEEEEEEEEEELP!!!!

	fn area_unassigned(&self, area: &Area) -> bool {
		self.0[area.min.y as usize .. area.max.y as usize].iter().all(|row| {
			row[area.min.x as usize .. area.max.x as usize].iter().all(|&tile| match tile {
				Tile::Unassigned => true,
				_ => false
			})
		})
	}

	pub fn area(&self) -> Area {
		Area::new(0, 0, self.0.len() as u32 - 1, self.0[0].len() as u32 - 1)
	}

	fn place(&mut self, tile: Tile, area: &Area) {
		for y in area.min.y as usize .. area.max.y as usize {
			for x in area.min.x as usize .. area.max.x as usize {
				self.0[y][x] = tile;
			}
		}
	}

	fn place_rooms(&mut self, rng: &mut rand::ThreadRng, n: u32, min_size: &Vector, max_size: &Vector) {
		let area = self.area();
		let mut id = 0;

		for _ in 1 .. n {
			let room_area = area.random_subarea(rng, min_size, max_size);
			if !self.area_unassigned(&room_area) {
				continue;
			}

			id = id + 1;
			self.place(Tile::Room(id), &room_area);
		}
	}

	pub fn to_string(&self) -> String {
		self.0.iter().map(|row| {
			let row_iter = row.iter().map(|tile| tile.to_char()).chain("\n".chars());
			
			row_iter.collect::<String>()
		}).collect()
	}
}

fn main() {
	let mut rng = rand::thread_rng();

	//let mut labyrinth: [[Tile; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH] = [[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH];

	//labyrinth[0][0] = Tile::Room(1);

	//let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	let mut labyrinth = Labyrinth([[Tile::Unassigned; LABYRINTH_HEIGHT]; LABYRINTH_WIDTH]);

	//let room_area = Area::new(2,2,16,16);
	//let subArea = room_area.random_subarea(&mut rng, &Vector::new(3,3), &Vector::new(5,5));

	labyrinth.place_rooms(&mut rng, 2, &Vector::new(3,3), &Vector::new(8,8));

	//labyrinth.place(Tile::Room(2), &room_area);

	//labyrinth.place(Tile::Tunnel(DirectionSet::new().set_down()), &subArea);

	println!("{}", labyrinth.to_string());

	//println!("{}", Area { min: Vector { x: 0, y: 9 }, max: Vector { x: 0, y: 9 } }.to_string());

	//println!("Room {} - Subarea {} / {}", room_area.to_string(), subArea.to_string(), subArea.dimensions().to_string());
}
