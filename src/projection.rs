pub struct Matrix4x4 {
    values: [f32; 16],
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {

        let mut res = Matrix4x4 {
            values: [0f32; 16],
        };

        res.identity();
        res
    }

    pub fn identity(&mut self) {
        // Set diagonal to 1
        [0, 5, 10, 15].iter().for_each(|i| {
            self.values[*i] = 0f32
        });

        // Set rest to 0
        [1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14].iter().for_each(|i| {
            self.values[*i] = 0f32
        });
    }
}
