#![allow(dead_code)]

extern crate rand;
use rand::Rng;
use std::cmp;
//use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;
use std::vec::Vec;
//use std::collections::BinaryHeap;

mod roomat;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
	Unassigned,
	Room(i32)
}

impl Tile {
	pub fn to_char(self) -> char {
		match self {
			Tile::Unassigned => '?',
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
	map: ([[Tile; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT])
}

impl Labyrinth {
	pub fn new() -> Labyrinth {
		Labyrinth { map: [[Tile::Unassigned; LABYRINTH_WIDTH]; LABYRINTH_HEIGHT] }
	}

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
			self.place(Tile::Room(id), &room_area);
		}
	}

	pub fn to_string(&self) -> String {
		self.map.iter().map(|row| {
			let row_iter = row.iter().map(|tile| tile.to_char()).chain("\n".chars());
			
			row_iter.collect::<String>()
		}).collect()
	}
}

fn main() {
	let mut rng = rand::thread_rng();

	let mut labyrinth = Labyrinth::new();

	labyrinth.place_rooms(&mut rng, 10, &Vector::new(3,3), &Vector::new(8,8));

	let mut subarea_a = roomat::SubArea::new();
	let mut subarea_b = roomat::SubArea::new();

	subarea_a.add_association(&subarea_b);

	let area_data = roomat::Area::Area(vec![]);

	println!("{}", labyrinth.to_string());

	println!("{}", roomat::too_strong());
}
