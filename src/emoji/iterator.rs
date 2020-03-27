use super::{GRID_ITEM_SIZE, TOTAL_COLUMNS, TOTAL_ROWS};
const STEP: u32 = 1;

pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub row: u32,
    pub column: u32,
}

pub struct PartsIterator {
    x_step: u32,
    y_step: u32,
}

impl PartsIterator {
    pub fn new() -> Self {
        PartsIterator {
            x_step: 0,
            y_step: 0,
        }
    }
}

impl Iterator for PartsIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.y_step) >= TOTAL_ROWS {
            return None;
        }

        let current_coordinate = Coordinate {
            x: self.x_step * GRID_ITEM_SIZE,
            y: self.y_step * GRID_ITEM_SIZE,
            row: self.y_step,
            column: self.x_step,
        };

        if self.x_step >= TOTAL_COLUMNS - 1 {
            self.x_step = 0;
            self.y_step += STEP;
        } else {
            self.x_step += STEP;
        }

        Some(current_coordinate)
    }
}
