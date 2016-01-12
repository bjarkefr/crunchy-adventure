#![allow(dead_code)]

extern crate rand;
use rand::Rng;
use std::cmp;
use std::ops::Add;
use std::ops::Sub;
//use std::cmp::Equiv;

#[derive(Copy, Clone, PartialEq)]
struct Direction(u32);

impl Direction {
	pub fn inverse(self) -> Direction {
		match self {
			DIRECTION_LEFT => DIRECTION_RIGHT,
			DIRECTION_RIGHT => DIRECTION_LEFT,
			DIRECTION_UP => DIRECTION_DOWN,
			DIRECTION_DOWN => DIRECTION_UP,
			_ => DIRECTION_NONE
		}
	}

	pub fn to_unit_vector(self) -> Vector {
		match self {
			DIRECTION_LEFT => Vector::new(-1, 0),
			DIRECTION_RIGHT => Vector::new(1, 0),
			DIRECTION_UP => Vector::new(0, -1),
			DIRECTION_DOWN => Vector::new(0, 1),
			_ => Vector::new(0, 0)
		}
	}
}

const DIRECTION_NONE : Direction = Direction(0);
const DIRECTION_LEFT : Direction = Direction(1);
const DIRECTION_RIGHT : Direction = Direction(2);
const DIRECTION_UP : Direction = Direction(4);
const DIRECTION_DOWN : Direction = Direction(8);

#[derive(Copy, Clone, PartialEq)]
struct DirectionSet(u32);

impl DirectionSet {
	pub fn new() -> DirectionSet {
		DirectionSet(0)
	}

	pub fn is_set(self, dir: Direction) -> bool {
		self.0 & dir.0 != 0
	}

	pub fn set(self, dir: Direction) -> DirectionSet {
		DirectionSet(self.0 | dir.0)
	}

	pub fn to_vec(self) -> Vec<Direction> {
		let mut vec = Vec::new();

		if self.is_set(DIRECTION_LEFT) {
			vec.push(DIRECTION_LEFT)
		}
		if self.is_set(DIRECTION_RIGHT) {
			vec.push(DIRECTION_RIGHT)
		}
		if self.is_set(DIRECTION_UP) {
			vec.push(DIRECTION_UP)
		}
		if self.is_set(DIRECTION_DOWN) {
			vec.push(DIRECTION_DOWN)
		}

		vec
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

#[derive(Copy, Clone, PartialEq)]
enum Tile {
	Unassigned,
	Tunnel(DirectionSet),
	Room(i32)
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

const LABYRINTH_WIDTH: usize = 60;
const LABYRINTH_HEIGHT: usize = 20;

struct Labyrinth([[Tile; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT]);

#[derive(Clone, PartialEq)]
struct Vector {
	x: i32,
	y: i32
}

impl Vector {
	pub fn new(x: i32, y: i32) -> Vector {
		Vector { x: x, y: y }
	}

	pub fn to_direction(&self) -> Direction {
		if self.x == 0 {
			if self.y == 1 { DIRECTION_DOWN } else { if self.y == -1 { DIRECTION_UP } else { DIRECTION_NONE } }
		}
		else if self.y == 0 {
			if self.x == 1 { DIRECTION_RIGHT } else { if self.x == -1 { DIRECTION_LEFT } else { DIRECTION_NONE } }
		}
		else {
			DIRECTION_NONE
		}
	}

	pub fn to_string(&self) -> String {
		format!("({}, {})", self.x, self.y)
	}
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y }
    }
}

const LEFT_UNIT_VECTOR : Vector = Vector { x: -1, y: 0 }; //DIRECTION_UP.to_unit_vector(); // impossible method call without some experimental features...
const RIGHT_UNIT_VECTOR : Vector = Vector { x: 1, y: 0 };
const UP_UNIT_VECTOR : Vector = Vector { x: 0, y: -1 };
const DOWN_UNIT_VECTOR : Vector = Vector { x: 0, y: 1 };

struct Area {
	min: Vector,
	max: Vector
}

impl Area {
	pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Area {
		Area { min: Vector { x: min_x, y: min_y }, max: Vector { x: max_x, y: max_y } }
	}

	pub fn dimensions(&self) -> Vector {
		Vector::new(self.max.x - self.min.x + 1, self.max.y - self.min.y + 1)
	}

	pub fn enlarge(&self, size: i32) -> Area {
		Area::new(self.min.x - size, self.min.y - size, self.max.x + size, self.max.y + size)
	}

	pub fn crop(&self, area: &Area) -> Area {
		Area::new(cmp::max(self.min.x, area.min.x), cmp::max(self.min.y, area.min.y), cmp::min(self.max.x, area.max.x), cmp::min(self.max.y, area.max.y))
	}

	pub fn random_subarea(&self, rng: &mut rand::ThreadRng, min_size: &Vector, max_size: &Vector) -> Area {
		let min = Vector {
			x: rng.gen_range(self.min.x, self.max.x - min_size.x + 2),
			y: rng.gen_range(self.min.y, self.max.y - min_size.y + 2)
		};

		let max = Vector {
			x: rng.gen_range(min.x + min_size.x - 1, cmp::min(self.max.x, min.x + max_size.x - 1) + 1),
			y: rng.gen_range(min.y + min_size.y - 1, cmp::min(self.max.y, min.y + max_size.y - 1) + 1)
		};

		Area { min: min, max: max }
	}

	pub fn contains(&self, pos: &Vector) -> bool {
		pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
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

	pub fn area_unassigned(&self, area: &Area) -> bool {
		self.0[area.min.y as usize .. (area.max.y + 1) as usize].iter().all(|row| {
			row[area.min.x as usize .. (area.max.x + 1) as usize].iter().all(|&tile| match tile {
				Tile::Unassigned => true,
				_ => false
			})
		})
	}

	pub fn area(&self) -> Area {
		Area::new(0, 0, self.0[0].len() as i32 - 1, self.0.len() as i32 - 1)
	}

	pub fn place(&mut self, tile: Tile, area: &Area) {
		for y in area.min.y as usize .. (area.max.y + 1) as usize {
			for x in area.min.x as usize .. (area.max.x + 1) as usize {
				self.0[y][x] = tile;
			}
		}
	}

	pub fn place_rooms(&mut self, rng: &mut rand::ThreadRng, n: i32, min_size: &Vector, max_size: &Vector) {
		let area = self.area();
		let mut id = 0;

		for _ in 0 .. n {
			let room_area = area.random_subarea(rng, min_size, max_size);
			if !self.area_unassigned(&area.crop(&room_area.enlarge(1))) {
				continue;
			}

			id += 1;
			self.place(Tile::Room(id), &room_area);
		}
	}

	fn dig_tunnel_segment(&mut self, from: &Vector, to: &Vector) {
		let dir = (to - from).to_direction();

		self.0[from.y as usize][from.x as usize] = match self.0[from.y as usize][from.x as usize] {
			Tile::Unassigned => Tile::Tunnel(DirectionSet::new().set(dir)),
			Tile::Tunnel(dir_set) => Tile::Tunnel(dir_set.set(dir)),
			_ => Tile::Unassigned
		};

		let inverse_dir = dir.inverse();

		self.0[to.y as usize][to.x as usize] = match self.0[to.y as usize][to.x as usize] {
			Tile::Unassigned => Tile::Tunnel(DirectionSet::new().set(inverse_dir)),
			Tile::Tunnel(dir_set) => Tile::Tunnel(dir_set.set(inverse_dir)),
			_ => Tile::Unassigned
		}
	}

	fn direction_unassigned(&self, from: &Vector, dir: Direction) -> bool {
		let dest = from + &dir.to_unit_vector();

		if !self.area().contains(&dest) {
			return false
		}

		self.0[dest.y as usize][dest.x as usize] == Tile::Unassigned
	}

	fn get_unassigned_directions(&self, from: &Vector) -> DirectionSet {
		let mut unassigned_directions = DirectionSet::new();

		if self.direction_unassigned(from, DIRECTION_LEFT) {
			unassigned_directions = unassigned_directions.set(DIRECTION_LEFT)
		}
		if self.direction_unassigned(from, DIRECTION_RIGHT) {
			unassigned_directions = unassigned_directions.set(DIRECTION_RIGHT)
		}
		if self.direction_unassigned(from, DIRECTION_UP) {
			unassigned_directions = unassigned_directions.set(DIRECTION_UP)
		}
		if self.direction_unassigned(from, DIRECTION_DOWN) {
			unassigned_directions = unassigned_directions.set(DIRECTION_DOWN)
		}

		unassigned_directions
	}

	pub fn place_tunnel(&mut self, rng: &mut rand::ThreadRng, start: &Vector) {
		let mut current = (*start).clone();

		loop {
			let dirs = self.get_unassigned_directions(&current).to_vec();
			if dirs.len() == 0 {
				return;
			}

			let dir = dirs[rng.gen_range(0, dirs.len())];

			let to = &current + &dir.to_unit_vector();
			self.dig_tunnel_segment(&current, &to);
			current = to;
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

	//labyrinth[0][0] = Tile::Room(1);

	//let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	let mut labyrinth = Labyrinth([[Tile::Unassigned; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT]);

	//let room_area = Area::new(2,2,16,16);
	//let subArea = room_area.random_subarea(&mut rng, &Vector::new(3,3), &Vector::new(5,5));

	labyrinth.place_rooms(&mut rng, 10, &Vector::new(3,3), &Vector::new(8,8));
	labyrinth.place_tunnel(&mut rng, &Vector::new(0,0));

	//labyrinth.place(Tile::Room(9), &Area::new(0,0,9,0));

	//labyrinth.place(Tile::Tunnel(DirectionSet::new().set_down()), &subArea);

	println!("{}", labyrinth.to_string());

	//println!("{}", Area { min: Vector { x: 0, y: 9 }, max: Vector { x: 0, y: 9 } }.to_string());

	//println!("Room {} - Subarea {} / {}", room_area.to_string(), subArea.to_string(), subArea.dimensions().to_string());
}
