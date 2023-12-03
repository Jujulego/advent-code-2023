use na::{point, Point2, vector};
use py::BBox;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub value: u32,
    pub position: Point2<i32>,
    pub size: i32,
}

impl Number {
    pub fn surroundings(&self) -> BBox<i32, 2> {
        let min = point![
            self.position.x - 1,
            self.position.y - 1
        ];

        BBox::from_anchor_size(&min, &vector![self.size + 2, 3])
    }
}