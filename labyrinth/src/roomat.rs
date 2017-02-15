// extend to graph

// enum Area<'a> {
//     SubArea(&'a [Area<'a>])
// }

pub enum Area {
    SubArea(SubArea),
    Room(Room)
}

impl Area {
    pub fn create_from_subarea(subarea: SubArea) -> Area {
        Area::SubArea(subarea)
    }

    pub fn create_from_room(room: Room) -> Area {
        Area::Room(room)
    }
}

pub struct Room {
    min_area: i32,
    min_side_length: i32
}

impl Room {
    pub fn new(min_area: i32, min_side_length: i32) -> Room {
        Room { min_area: min_area, min_side_length: min_side_length }
    }
}

pub struct SubArea {
    pub locations: Vec<Area>,
    pub associations: Vec<Association>
}

impl SubArea {
    pub fn new(locations: Vec<Area>, associations: Vec<Association>) -> SubArea {
        SubArea { locations: locations, associations: associations }
    }
}

pub struct Association {
    pub locations: (usize, usize),
    pub weight: f32
}

impl Association {
    pub fn new(location_a: usize, location_b: usize, weight: f32) -> Association {
        Association { locations: (location_a, location_b), weight: weight }
    }
}

// impl<'a> SubArea<'a> {
//     pub fn new() -> SubArea<'a> {
//         SubArea {
//             associations: Vec::new()
//         }
//     }

//     pub fn add_association(&mut self, target: &'a SubArea<'a>) {
//         self.associations.push(target);
//     }
// }

pub fn too_strong() -> String
{
    return String::from("Hi!");
}
