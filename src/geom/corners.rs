use super::*;

/// A container with components for the four corners of a rectangle.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Corners<T> {
    /// The value for the top left corner.
    pub top_left: T,
    /// The value for the top right corner.
    pub top_right: T,
    /// The value for the bottom right corner.
    pub bottom_right: T,
    /// The value for the bottom left corner.
    pub bottom_left: T,
}

impl<T> Corners<T> {
    /// Create a new instance from the four components.
    pub const fn new(top_left: T, top_right: T, bottom_right: T, bottom_left: T) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }

    /// Create an instance with four equal components.
    pub fn splat(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            top_left: value.clone(),
            top_right: value.clone(),
            bottom_right: value.clone(),
            bottom_left: value,
        }
    }

    /// Map the individual fields with `f`.
    pub fn map<F, U>(self, mut f: F) -> Corners<U>
    where
        F: FnMut(T) -> U,
    {
        Corners {
            top_left: f(self.top_left),
            top_right: f(self.top_right),
            bottom_right: f(self.bottom_right),
            bottom_left: f(self.bottom_left),
        }
    }

    /// Zip two instances into one.
    pub fn zip<U>(self, other: Corners<U>) -> Corners<(T, U)> {
        Corners {
            top_left: (self.top_left, other.top_left),
            top_right: (self.top_right, other.top_right),
            bottom_right: (self.bottom_right, other.bottom_right),
            bottom_left: (self.bottom_left, other.bottom_left),
        }
    }

    /// An iterator over the corners, starting with the top left corner,
    /// clockwise.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.top_left, &self.top_right, &self.bottom_right, &self.bottom_left]
            .into_iter()
    }

    /// Whether all sides are equal.
    pub fn is_uniform(&self) -> bool
    where
        T: PartialEq,
    {
        self.top_left == self.top_right
            && self.top_right == self.bottom_right
            && self.bottom_right == self.bottom_left
    }
}

impl<T> Get<Corner> for Corners<T> {
    type Component = T;

    fn get(self, corner: Corner) -> T {
        match corner {
            Corner::TopLeft => self.top_left,
            Corner::TopRight => self.top_right,
            Corner::BottomRight => self.bottom_right,
            Corner::BottomLeft => self.bottom_left,
        }
    }

    fn get_mut(&mut self, corner: Corner) -> &mut T {
        match corner {
            Corner::TopLeft => &mut self.top_left,
            Corner::TopRight => &mut self.top_right,
            Corner::BottomRight => &mut self.bottom_right,
            Corner::BottomLeft => &mut self.bottom_left,
        }
    }
}

/// The four corners of a rectangle.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Corner {
    /// The top left corner.
    TopLeft,
    /// The top right corner.
    TopRight,
    /// The bottom right corner.
    BottomRight,
    /// The bottom left corner.
    BottomLeft,
}
