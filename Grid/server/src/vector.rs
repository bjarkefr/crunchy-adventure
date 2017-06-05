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
