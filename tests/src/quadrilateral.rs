use std::ops::AddAssign;

#[derive(Debug)]
pub struct Quadrilateral<T>
where
    T : PartialOrd + AddAssign + PartialOrd<i32>
{
    length: T,
    width: T
}

impl<T> Quadrilateral<T>
where
    T : PartialOrd + AddAssign + PartialOrd<i32>
{
    pub fn new(len: T, width: T) -> Self {
        if len > 100 || width > 100 {
            panic!("The maximum dimension is 100x100")
        }
        Quadrilateral { length: len, width: width }
    }

    pub fn can_hold(&self, other: &Quadrilateral<T>) -> bool {
        self.length > other.length && self.width > other.width
    }

    // Consumes the given Quadrilateral
    pub fn merge(&mut self, other: Quadrilateral<T>) {
        self.length += other.length;
        self.width += other.width;
    }

    // Consumes both the of the Quadrilateral
    pub fn merge_to_new(self, other: Quadrilateral<T>) -> Quadrilateral<T> {
        let mut new = Quadrilateral {
            ..self
        };

        new.merge(other);

        new
    }

    pub fn length(&self) -> &T {
        &self.length
    }
    pub fn width(&self) -> &T {
        &self.width
    }
}
