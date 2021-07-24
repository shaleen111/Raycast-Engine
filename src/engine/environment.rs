use sdl2::pixels::Color;

use std::collections::HashMap;
pub struct Environment
{
    width: usize,
    height: usize,

    map: Vec<Vec<usize>>,
    resource_lookup: HashMap<usize, Color>,
}

impl Environment
{
    pub fn new(width: usize, height: usize, map: Vec<Vec<usize>>) -> Self
    {
        Self
        {
            width,
            height,
            map,
            resource_lookup: HashMap::new(),
        }
    }

    pub fn get_object_in_map(&self, x: isize, y: isize) -> usize
    {
        if x < 0 || y < 0
           || x >= self.width as isize
           || y >= self.height as isize
        {
            return 1;
        }

        return self.map[y as usize][x as usize];
    }

    pub fn get_resource(&self, index: usize) -> Option<Color>
    {
        match self.resource_lookup.get(&index)
        {
            Some(&val) =>
            {
                Some(val)
            },
            None => None,
        }
    }

    pub fn add_resource(&mut self, index: usize, col: Color)
    {
        self.resource_lookup.insert(index, col);
    }
}
