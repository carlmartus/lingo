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

    pub fn ortho(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {

        self.values[ 0] = 2f32 / (x1-x0);
        self.values[ 5] = 2f32 / (y1-y0);
        self.values[10] = 1f32;
        self.values[15] = 1f32;

        self.values[12] = -(x1+x0)/(x1-x0);
        self.values[13] = -(y1+y0)/(y1-y0);
        self.values[14] = 0f32;

        // Set rest to 0
        [1, 2, 3, 4, 6, 7, 8, 9].iter().for_each(|i| {
            self.values[*i] = 0f32;
        });
    }

    pub fn perspective(&mut self, fov: f32, ratio: f32, near: f32, far: f32) {
        let size = near * (fov * 0.5).tan();
        let left = -size;
        let right = size;
        let bottom = -size / ratio;
        let top = size / ratio;

        self.values[0] = 2f32 * near / (right - left);
        self.values[5] = 2f32 * near / (top - bottom);
        self.values[8] = (right + left) / (right - left);
        self.values[9] = (top + bottom) / (top - bottom);
        self.values[10] = -(far + near) / (far - near);
        self.values[11] = -1f32;
        self.values[14] = -(2f32 * far * near) / (far - near);

        // Set rest to 0
        [1, 2, 3, 4, 6, 7, 12, 13, 15].iter().for_each(|i| {
            self.values[*i] = 0f32;
        });
    }
}
