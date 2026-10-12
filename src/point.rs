//! Point.rs
//! 
//! Defines a point on an xy coordinate grid
//! 

/// A point in 2d space
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Default for Point<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Point<T> {
    /// Sets the value of x
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    /// Sets the value of y
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    /// Sets the value of x and y
    pub fn set_xy(
        &mut self,
        x: T,
        y: T,
    ) {
        self.x = x;
        self.y = y;
    }
}

impl<T> Point<T>
where
    T: Clone
{
    /// Gets the value of x
    pub fn get_x(&self) -> T {
        self.x.clone()
    }

    /// Gets the value of y
    pub fn get_y(&self) -> T {
        self.y.clone()
    }
}
