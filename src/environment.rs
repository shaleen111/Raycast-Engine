pub struct Environment
{
    width: usize,
    height: usize,

    map: Vec<Vec<usize>>,
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

    // pub fn get_resource(&self, index: usize) -> [usize; 4]
    // {
    //     if index > self.resource_lookup.len()
    //     {
    //         panic!("Out of Bounds Resource Lookup");
    //     }

    //     return self.resource_lookup[index];
    // }
}
