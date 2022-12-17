// source: https://github.com/12101111/yolo-rs/blob/master/src/yolo.rs
// x, y is the upper left corner
/// A bounding box struct with mergre functions
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    /// left side
    pub x: f32,
    /// upper side
    pub y: f32,
    /// width
    pub w: f32,
    /// height
    pub h: f32,
}
