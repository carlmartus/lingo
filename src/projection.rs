pub struct Matrix4x4 {
    pub values: [f32; 16],
}

#[derive(Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    fn normalize(&self) -> Vec3 {
        let r = (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt();
        if r == 0f32 {
            self.clone()
        } else {
            let r = 1f32 / r;
            Vec3(
                self.0*r,
                self.1*r,
                self.2*r)
        }
    }

    fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3(
            a.1*b.2 - a.2*b.1,
            a.2*b.0 - a.0*b.2,
            a.0*b.1 - a.1*b.0)
    }
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {

        let mut res = Matrix4x4 {
            values: [0f32; 16],
        };

        res.identity();
        res
    }

    pub fn get_xy(&self, x: usize, y: usize) -> f32 {
        self.values[x + y*4]
    }

    pub fn identity(&mut self) {
        // Set diagonal to 1
        [0, 5, 10, 15].iter().for_each(|i| {
            self.values[*i] = 1f32
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

    pub fn multiply(a: &Matrix4x4, b: &Matrix4x4) -> Matrix4x4 {

        let mut values = [0f32; 16];

        let mut i = 0;
        for y in 0..4 {
            for x in 0..4 {
                let r = y << 2;
                values[i] =
                    a.values[r+ 0]*b.values[x+ 0] +
                    a.values[r+ 1]*b.values[x+ 4] +
                    a.values[r+ 2]*b.values[x+ 8] +
                    a.values[r+ 3]*b.values[x+12];
                i = i+1;
            }
        }

        Matrix4x4 { values }
    }

    pub fn look_at(&mut self, eye: Vec3, at: Vec3, up: Vec3) {
        let forw = Vec3(
            at.0 - eye.0,
            at.1 - eye.1,
            at.2 - eye.2);

        let forw = forw.normalize();

        let side = Vec3::cross(&up, &forw).normalize();

        let up = Vec3::cross(&forw, &side);

        let mut m0 = Matrix4x4::new();

        m0.values[ 0] = side.0;
        m0.values[ 4] = side.1;
        m0.values[ 8] = side.2;

        m0.values[ 1] = up.0;
        m0.values[ 5] = up.1;
        m0.values[ 9] = up.2;

        m0.values[ 2] = -forw.0;
        m0.values[ 6] = -forw.1;
        m0.values[10] = -forw.2;

        let mut m1 = Matrix4x4::new();
        m1.values[12] = -eye.0;
        m1.values[13] = -eye.1;
        m1.values[14] = -eye.2;

        *self = Matrix4x4::multiply(&m1, &m0);
    }

    pub fn camera_3d(&mut self,
                     fov: f32, ratio: f32, near: f32, far: f32,
                     eye: Vec3, at: Vec3, up: Vec3) {

        // Perspective matrix
        let mut perspective = Matrix4x4::new();
        perspective.perspective(fov, ratio, near, far);

        let mut look = Matrix4x4::new();
        look.look_at(eye, at, up);

        *self = Matrix4x4::multiply(&look, &perspective);
    }
}
