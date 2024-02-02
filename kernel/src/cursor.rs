pub struct Cursor {
    x: usize,
    max: usize,
}

impl Cursor {
    pub fn new(max: usize) -> Self {
        Cursor { x: 0, max }
    }

    pub fn x(&self) -> usize {
        self.x
    }
}

impl Iterator for Cursor {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.x + 1 > self.max {
            return None;
        }
        self.x += 1;
        Some(())
    }
}

impl DoubleEndedIterator for Cursor {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }
        self.x -= 1;
        Some(())
    }
}

pub struct Cursor2D {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Cursor2D {
    pub fn new(width: usize, height: usize) -> Self {
        Cursor2D {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn y_with(&mut self, f: impl FnOnce(usize) -> usize) -> Result<(), CursorError> {
        let new_y = f(self.y);
        if new_y > self.height {
            return Err(CursorError::OutOfBounds);
        }
        self.y = new_y;
        Ok(())
    }

    pub fn x_with(&mut self, f: impl FnOnce(usize) -> usize) -> Result<(), CursorError> {
        let new_x = f(self.x);
        if new_x > self.width {
            return Err(CursorError::OutOfBounds);
        }
        self.x = new_x;
        Ok(())
    }

    pub fn next_line(&mut self) -> Option<()> {
        if self.y >= self.height {
            return None;
        }
        self.y += 1;
        self.x = 0;
        Some(())
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Iterator for Cursor2D {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x >= self.width && self.y >= self.height {
            return None;
        } else if self.x >= self.width {
            self.y += 1;
            self.x = 0;
        }
        Some(())
    }
}

impl DoubleEndedIterator for Cursor2D {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == 0 && self.y == 0 {
            return None;
        } else if self.x == 0 {
            self.y -= 1;
            self.x = self.width;
        }
        Some(())
    }
}

#[derive(Debug)]
pub enum CursorError {
    OutOfBounds,
}
