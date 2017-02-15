// extend to graph

// enum Area<'a> {
//     SubArea(&'a [Area<'a>])
// }

pub enum Area<'a> {
    Area(Vec<&'a SubArea<'a>>),
    Room
}

pub struct SubArea<'a> {
    associations: Vec<&'a SubArea<'a>>
}

impl<'a> SubArea<'a> {
    pub fn new() -> SubArea<'a>
    {
        SubArea {
            associations: Vec::new()
        }
    }

    pub fn add_association(&mut self, target: &'a SubArea<'a>)
    {
        self.associations.push(target);
    }
}

pub fn too_strong() -> String
{
    return String::from("Hi!");
}
