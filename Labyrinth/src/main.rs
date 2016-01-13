#![allow(dead_code)]

extern crate rand;
use rand::Rng;
use std::cmp;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;
use std::vec::Vec;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq, Eq)]
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

	pub fn to_char(self) -> char {
		match self.0 {
			0 => ' ',
			1 => '←',
			2 => '→',
			4 => '↑',
			8 => '↓',
			_ => '?'
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

const LABYRINTH_WIDTH: usize = 40;
const LABYRINTH_HEIGHT: usize = 40;

struct Labyrinth {
	map: ([[Tile; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT]),
	connected_rooms: Vec<bool>
}

impl Labyrinth {
	pub fn new() -> Labyrinth {
		Labyrinth { map: [[Tile::Unassigned; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT], connected_rooms: Vec::new() }
	}

	/*fn iter(&self, area: Area) -> Box<Iterator<Item=Tile>> { //iter::Map<iter::Take<iter::Skip<slice::Iter<Tile>>>> {
		let rowIter = self.map.iter().skip(area.min.y as usize).take((area.max.y - area.min.y) as usize);

		Box::new(rowIter.map(|row| row.iter().skip(area.min.x as usize).take((area.max.x - area.min.x) as usize)))
	}*/ // @TOBY: HEEEEEEEEEEELP!!!!

	pub fn area_unassigned(&self, area: &Area) -> bool {
		self.map[area.min.y as usize .. (area.max.y + 1) as usize].iter().all(|row| {
			row[area.min.x as usize .. (area.max.x + 1) as usize].iter().all(|&tile| match tile {
				Tile::Unassigned => true,
				_ => false
			})
		})
	}

	pub fn area(&self) -> Area {
		Area::new(0, 0, self.map[0].len() as i32 - 1, self.map.len() as i32 - 1)
	}

	pub fn place(&mut self, tile: Tile, area: &Area) {
		for y in area.min.y as usize .. (area.max.y + 1) as usize {
			for x in area.min.x as usize .. (area.max.x + 1) as usize {
				self.map[y][x] = tile;
			}
		}
	}

	pub fn place_rooms(&mut self, rng: &mut rand::ThreadRng, n: i32, min_size: &Vector, max_size: &Vector) {
		let area = self.area();
		let mut id = -1;

		for _ in 0 .. n {
			let room_area = area.random_subarea(rng, min_size, max_size);
			if !self.area_unassigned(&area.crop(&room_area.enlarge(1))) {
				continue;
			}

			id += 1;
			self.connected_rooms.push(false);
			self.place(Tile::Room(id), &room_area);
		}
	}

	fn dig_tunnel_piece(&mut self, pos: &Vector, dir: Direction) {
		self.map[pos.y as usize][pos.x as usize] = match self.map[pos.y as usize][pos.x as usize] {
			Tile::Unassigned => Tile::Tunnel(DirectionSet::new().set(dir)),
			Tile::Tunnel(dir_set) => Tile::Tunnel(dir_set.set(dir)),
			Tile::Room(n) => {
				self.connected_rooms[n as usize] = true;
				Tile::Room(n)
			}
		}
	}

	fn dig_tunnel_segment(&mut self, from: &Vector, dir: Direction) -> Vector {
		let to = from + &dir.to_unit_vector();
		let inverse_dir = dir.inverse();

		self.dig_tunnel_piece(from, dir);
		self.dig_tunnel_piece(&to, inverse_dir);

		to
	}

	fn direction_digable(&self, from: &Vector, dir: Direction) -> bool {
		let dest = from + &dir.to_unit_vector();

		if !self.area().contains(&dest) {
			return false
		}

		match self.map[dest.y as usize][dest.x as usize] {
			Tile::Unassigned => true,
			Tile::Room(n) => !self.connected_rooms[n as usize],
			_ => false
		}
	}

	fn digable_directions(&self, from: &Vector) -> DirectionSet {
		if match self.map[from.y as usize][from.x as usize] {
			Tile::Room(_) => true,
			_ => false
		} {
			return DirectionSet::new()
		}

		let mut digable_directions = DirectionSet::new();

		if self.direction_digable(from, DIRECTION_LEFT) {
			digable_directions = digable_directions.set(DIRECTION_LEFT)
		}
		if self.direction_digable(from, DIRECTION_RIGHT) {
			digable_directions = digable_directions.set(DIRECTION_RIGHT)
		}
		if self.direction_digable(from, DIRECTION_UP) {
			digable_directions = digable_directions.set(DIRECTION_UP)
		}
		if self.direction_digable(from, DIRECTION_DOWN) {
			digable_directions = digable_directions.set(DIRECTION_DOWN)
		}

		digable_directions
	}

	fn tunnel_in_direction(&mut self, rng: &mut rand::ThreadRng, pile: &mut RandomPile, start: &Vector, dir: Direction) {
		let mut current = (*start).clone();
		let mut current_dir = dir;

		if !self.direction_digable(&current, dir) {
			return;
		}

		loop {
			current = self.dig_tunnel_segment(&current, current_dir);

			let mut dirs = self.digable_directions(&current).to_vec();
			if dirs.len() == 0 {
				return;
			}

			let max_n = dirs.len();
			current_dir = dirs.swap_remove(rng.gen_range(0, max_n));

			pile.push_multiple(rng,
				dirs.iter().map(|dir| BasedDir::new(current, *dir)).collect()
			);
		}
	}

	/*fn find_tunnel_entrance(&self, rng: &mut rand::ThreadRng) -> BasedDir {
		BasedDir::new(Vector::new(0,0), DIRECTION_RIGHT) // TODO!!
	}*/

	pub fn place_tunnels(&mut self, rng: &mut rand::ThreadRng, start: BasedDir) {
		self.dig_tunnel_piece(&start.pos, start.dir.inverse());

		let mut pile = RandomPile::new();

		pile.push(rng, start);

		while match pile.pop() {
			Some(dir) => {
				self.tunnel_in_direction(rng, &mut pile, &dir.pos, dir.dir);
				true
			},
			None => false
		} {}
	}

	pub fn to_string(&self) -> String {
		self.map.iter().map(|row| {
			let row_iter = row.iter().map(|tile| tile.to_char()).chain("\n".chars());
			
			row_iter.collect::<String>()
		}).collect()
	}

	pub fn to_string_large(&self) -> String {
		let area = self.area();
		let mut output = String::new();

		output.push_str("#");
		for x in area.min.x .. area.max.x + 1 {
			output.push_str(match self.map[0][x as usize] {
				Tile::Tunnel(dir_set) => if dir_set.is_set(DIRECTION_UP) { " #" } else { "##" },
				_ => "##"
			});
		}
		output.push_str("\n");

		for y in area.min.y .. area.max.y + 1 {
			output.push_str(match self.map[y as usize][0] {
				Tile::Tunnel(dir_set) => if dir_set.is_set(DIRECTION_LEFT) { " " } else { "#" },
				_ => "#"
			});

			for x in area.min.x .. area.max.x + 1 {
				match self.map[y as usize][x as usize] {
					Tile::Tunnel(dir_set) => {
						output.push_str(" ");
						if dir_set.is_set(DIRECTION_RIGHT) {
							if x < area.max.x {
								match self.map[y as usize][(x + 1) as usize] {
									Tile::Room(_) => output.push_str("?"),
									_ => output.push_str(" ")
								}
							} else {
								output.push_str(" ")
							}
						} else {
							output.push_str("#");
						}
					},
					Tile::Room(_) => {
						if x < area.max.x {
							match self.map[y as usize][(x + 1) as usize] {
								Tile::Room(_) => output.push_str("  "),
								Tile::Tunnel(dir_set) => if dir_set.is_set(DIRECTION_LEFT) {
									output.push_str(" ?")
								} else {
									output.push_str(" #")
								},
								_ => output.push_str(" #")
							}
						} else {
							output.push_str(" #");
						}
					},
					_ => ()
				}
			};
			output.push_str("\n#");

			for x in area.min.x .. area.max.x + 1 {
				match self.map[y as usize][x as usize] {
					Tile::Tunnel(dir_set) => {
						if dir_set.is_set(DIRECTION_DOWN) {
							if y < area.max.y {
								match self.map[(y + 1) as usize][x as usize] {
									Tile::Room(_) => output.push_str("?"),
									_ => output.push_str(" ")
								}
							} else {
								output.push_str(" ")
							}
						} else {
							output.push_str("#")
						}
						output.push_str("#")
					},
					Tile::Room(_) => {
						if y < area.max.y {
							match self.map[(y + 1) as usize][x as usize] {
								Tile::Room(_) => {
									output.push_str(" ");
									if x < area.max.x {
										match self.map[(y + 1) as usize][(x + 1) as usize] {
											Tile::Room(_) => output.push_str(" "),
											_ => output.push_str("#")
										}
									} else {
										output.push_str("#");
									}
								},
								Tile::Tunnel(dir_set) => if dir_set.is_set(DIRECTION_UP) {
									output.push_str("?#")
								} else {
									output.push_str("##");
								},
								_ => output.push_str("##")
							}
						} else {
							output.push_str("##");
						}
					},
					_ => ()
				}
			};
			output.push_str("\n");
		}

		output
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct BasedDir {
	pos: Vector,
	dir: Direction
}

impl BasedDir {
	pub fn new(pos: Vector, dir: Direction) -> BasedDir {
		BasedDir { pos: pos, dir: dir }
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RandomNode {
	data: BasedDir,
	value: i32
}

impl Ord for RandomNode {
    fn cmp(&self, other: &RandomNode) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for RandomNode {
    fn partial_cmp(&self, other: &RandomNode) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

struct RandomPile(BinaryHeap<RandomNode>);

impl RandomPile {
	pub fn new() -> RandomPile {
		RandomPile(BinaryHeap::new())
	}

	pub fn push(&mut self, rng: &mut rand::ThreadRng, data: BasedDir) {
		let node = RandomNode { data: data, value: rng.gen() };
		self.0.push(node)
	}

	pub fn push_multiple(&mut self, rng: &mut rand::ThreadRng, data: Vec<BasedDir>) {
		for entry in data {
			self.push(rng, entry)
		}
	}

	pub fn pop(&mut self) -> Option<BasedDir> {
		match self.0.pop() {
			Some(node) => Some(node.data),
			None => None
		}
	}
}

fn main() {
	let mut rng = rand::thread_rng();

	//let mut heap = BinaryHeap::new();

	/*heap.push(RandomNode { pos: Vector::new(1, 0), value: 1 });
	heap.push(RandomNode { pos: Vector::new(2, 2), value: 5 });
	heap.push(RandomNode { pos: Vector::new(2, 2), value: 2 });

	match heap.pop() {
		Some(node) => println!("{}@{}", node.value, node.pos.to_string()),
		_ => ()
	};

	match heap.pop() {
		Some(node) => println!("{}@{}", node.value, node.pos.to_string()),
		_ => ()
	};*/

	//labyrinth[0][0] = Tile::Room(1);

	//let h = Tile::Tunnel(DirectionSet::new().set_up().set_down().set_right());

	let mut labyrinth = Labyrinth::new();

	labyrinth.place_rooms(&mut rng, 10, &Vector::new(3,3), &Vector::new(8,8));

	labyrinth.place_tunnels(&mut rng, BasedDir::new(Vector::new(0,0), DIRECTION_DOWN));

	//labyrinth.tunnel_in_direction(&mut rng, &Vector::new(0,0), DIRECTION_DOWN);

	//labyrinth.place(Tile::Room(9), &Area::new(0,0,9,0));

	//labyrinth.place(Tile::Tunnel(DirectionSet::new().set_down()), &subArea);

	println!("{}", labyrinth.to_string());
	println!("{}", labyrinth.to_string_large());

	//println!("{}", Area { min: Vector { x: 0, y: 9 }, max: Vector { x: 0, y: 9 } }.to_string());

	//println!("Room {} - Subarea {} / {}", room_area.to_string(), subArea.to_string(), subArea.dimensions().to_string());
}
