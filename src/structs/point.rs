#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct Point(pub i16, pub i16);

impl Point {
    pub fn neighboring_points(&self) -> Vec<Point> {
        vec![
            Point(self.0 - 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 + 1, self.1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}
