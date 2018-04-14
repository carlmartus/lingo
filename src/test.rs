use projection::Matrix4x4;

#[test]
fn projection_new() {
    let m = Matrix4x4::new();
    assert_identity(&m);
}
