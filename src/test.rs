use projection::Matrix4x4;

fn is_identity(mat: &Matrix4x4) -> bool {
    let mut good = true;

    [(0, 0), (1, 1), (2, 2), (3, 3)].iter().for_each(|&(x, y)| {
        if mat.get_xy(x, y) != 1f32 {
            good = false;
        }
    });

    [(0, 1), (0, 2), (0, 3)].iter().for_each(|&(x, y)| {
        if mat.get_xy(x, y) != 0f32 {
            good = false;
        }
    });

    good
}

#[test]
fn projection_new() {
    let m = Matrix4x4::new();
    assert!(is_identity(&m));
}

#[test]
fn projection_identity() {
    let mut m = Matrix4x4::new();
    m.identity();
    assert!(is_identity(&m));
}
