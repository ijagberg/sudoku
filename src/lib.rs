use std::fmt::Debug;

use simple_grid::Grid;

pub enum CreateSudokuError {
    InvalidSize,
    InvalidSecWidth,
    InvalidSecHeight,
    InvalidCombination,
}

impl Debug for CreateSudokuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let output = match self {
            CreateSudokuError::InvalidSize => "'size' must be a positive integer",
            CreateSudokuError::InvalidSecWidth => "'sec_width' must be a positive integer",
            CreateSudokuError::InvalidSecHeight => "'sec_height' must be a positive integer",
            CreateSudokuError::InvalidCombination => {
                "'size' must be equal to 'sec_width' * 'sec_height'"
            }
        };
        writeln!(f, "{}", output)
    }
}

#[derive(Clone, PartialEq)]
pub struct Sudoku {
    size: usize,
    sec_width: usize,
    sec_height: usize,
    grid: Grid<Option<u32>>,
}

impl Sudoku {
    /// Create a new instance of a Sudoku puzzle
    ///
    /// `size` is the number of unique values that should
    /// occupy each row, column, and section. A normal Sudoku puzzle
    /// has `size=9`
    ///
    /// `sec_width` is the width of the sections in the puzzle.
    /// `sec_height` is the height of the sections in the puzzle.
    /// A normal Sudoku puzzle has `sec_width=sec_height=3`.
    pub fn new(
        size: usize,
        sec_width: usize,
        sec_height: usize,
    ) -> Result<Self, CreateSudokuError> {
        if size < 1 {
            Err(CreateSudokuError::InvalidSize)
        } else if sec_width < 1 {
            Err(CreateSudokuError::InvalidSecWidth)
        } else if sec_height < 1 {
            Err(CreateSudokuError::InvalidSecHeight)
        } else if sec_width * sec_height != size {
            Err(CreateSudokuError::InvalidCombination)
        } else {
            Ok(Self {
                size,
                sec_width,
                sec_height,
                grid: Grid::new(size, size, vec![None; size * size]),
            })
        }
    }

    pub fn get(&self, col: usize, row: usize) -> Option<u32> {
        self.grid[(col, row)]
    }

    /// Set the value of the square at column `col`, row `row` to equal `v`
    /// # Panics
    /// If `v` is `Some(x)` where `x` is not in `(1..=size)`
    pub fn set(&mut self, col: usize, row: usize, v: Option<u32>) {
        match v {
            Some(v) => {
                if v as usize <= self.size && v >= 1 {
                    self.grid[(col, row)] = Some(v);
                } else {
                    panic!("invalid value: '{}'", v);
                }
            }
            None => {
                self.grid[(col, row)] = v;
            }
        }
    }

    pub fn populate_from_str(&mut self, s: &str) {
        let lines: Vec<&str> = s.split('\n').collect();
        assert_eq!(self.size, lines.len());
        for (row, cols) in lines
            .iter()
            .enumerate()
            .map(|(row, line)| (row, line.split_whitespace().collect::<Vec<_>>()))
        {
            assert_eq!(self.size, cols.len());
            for (col, &value) in cols.iter().enumerate() {
                let entry = match value {
                    "_" => None,
                    word => match word.parse::<u32>() {
                        Ok(v) if v > 0 && v as usize <= self.size => Some(v),
                        Ok(v) => panic!(
                            "{} is not a valid value for a Sudoku with size {}",
                            v, self.size
                        ),
                        Err(_) => panic!("Invalid format of populating string"),
                    },
                };
                self.set(col, row, entry);
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = usize> {
        self.grid.rows()
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn columns(&self) -> impl DoubleEndedIterator<Item = usize> {
        self.grid.columns()
    }

    pub fn sec_height(&self) -> usize {
        self.sec_height
    }

    pub fn sec_width(&self) -> usize {
        self.sec_width
    }

    pub fn is_solved(&self) -> bool {
        for v in 1..=self.size {
            // Check rows
            for row in self.grid.rows() {
                if self.count_in_row(row, v as u32) != 1 {
                    return false;
                }
            }

            // Check cols
            for col in self.grid.columns() {
                if self.count_in_col(col, v as u32) != 1 {
                    return false;
                }
            }

            // Check secs
            for row in self.grid.rows().step_by(self.sec_height) {
                for col in self.grid.columns().step_by(self.sec_width) {
                    if self.count_in_sec(col, row, v as u32) != 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn count_in_row(&self, row: usize, value: u32) -> usize {
        self.row_iter(row)
            .filter(|&value_in_row| value_in_row == &Some(value))
            .count()
    }

    fn count_in_col(&self, col: usize, value: u32) -> usize {
        self.col_iter(col)
            .filter(|&value_in_col| value_in_col == &Some(value))
            .count()
    }

    fn count_in_sec(&self, col: usize, row: usize, value: u32) -> usize {
        self.sec_iter(col, row)
            .filter(|&value_in_sec| value_in_sec == Some(value))
            .count()
    }

    pub fn can_place_value(&self, col: usize, row: usize, value: u32) -> bool {
        if self.get(col, row) == Some(value) {
            true
        } else {
            (self.count_in_col(col, value)
                + self.count_in_row(row, value)
                + self.count_in_sec(col, row, value))
                == 0
        }
    }

    pub fn place_if_possible(&mut self, col: usize, row: usize, value: u32) -> Result<(), ()> {
        if self.can_place_value(col, row, value) {
            self.set(col, row, Some(value));
            Ok(())
        } else {
            Err(())
        }
    }

    /// Create an iterator over the rows in a given column.
    /// Goes from top to bottom.
    pub fn col_iter(&self, col: usize) -> impl DoubleEndedIterator<Item = &Option<u32>> {
        self.grid.column_iter(col)
    }

    /// Create an iterator over the columns in a given row.
    /// Goes from left to right.
    pub fn row_iter(&self, row: usize) -> impl DoubleEndedIterator<Item = &Option<u32>> {
        self.grid.row_iter(row)
    }

    pub fn sec_iter(&self, col: usize, row: usize) -> SudokuSecIter {
        let first_row_in_sec = (row / self.sec_height) * self.sec_height;
        let first_col_in_sec = (col / self.sec_width) * self.sec_width;
        SudokuSecIter::new(first_row_in_sec, first_col_in_sec, &self)
    }
}

impl std::fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::iter;
        let value_length = self.size.to_string().len();

        let row_outputs: Vec<String> = {
            self.grid
                .rows()
                .map(|row| {
                    self.grid
                        .row_iter(row)
                        .enumerate()
                        .flat_map(|(col_index, col_value)| {
                            let col_value_output = match col_value {
                                Some(c) => iter::repeat(" ".to_string())
                                    .take(value_length - c.to_string().len())
                                    .chain(iter::once(c.to_string()))
                                    .collect::<String>(),
                                None => iter::repeat("─").take(value_length).collect::<String>(),
                            };
                            if col_index > 0 && col_index % self.sec_width == 0 {
                                // Border between two secs
                                vec!["│".to_string(), col_value_output]
                            } else {
                                vec![col_value_output]
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .map(|row_string| format!("│ {} │", row_string))
                .collect::<Vec<String>>()
        };

        let mut row = 0;
        let row_length = row_outputs[0].chars().count();
        while row < self.height() {
            if row % self.sec_height == 0 {
                if row == 0 {
                    // Top border
                    writeln!(
                        f,
                        "{}",
                        row_outputs[0]
                            .chars()
                            .enumerate()
                            .map(|(idx, c)| match c {
                                '│' if idx == 0 => '┌',
                                '│' if idx == row_length - 1 => '┐',
                                '│' => '┬',
                                _ => '─',
                            })
                            .collect::<String>()
                    )?;
                } else {
                    // Middle border(s)
                    writeln!(
                        f,
                        "{}",
                        row_outputs[0]
                            .chars()
                            .enumerate()
                            .map(|(idx, c)| match c {
                                '│' if idx == 0 => '├',
                                '│' if idx == row_length - 1 => '┤',
                                '│' => '┼',
                                _ => '─',
                            })
                            .collect::<String>()
                    )?;
                }
            }

            writeln!(f, "{}", row_outputs[row])?;
            row += 1;

            if row % self.sec_height == 0 && row == self.height() {
                // Bottom border
                writeln!(
                    f,
                    "{}",
                    row_outputs[0]
                        .chars()
                        .enumerate()
                        .map(|(idx, c)| match c {
                            '│' if idx == 0 => '└',
                            '│' if idx == row_length - 1 => '┘',
                            '│' => '┴',
                            _ => '─',
                        })
                        .collect::<String>()
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SudokuSecIter<'a> {
    start_row: usize,
    start_col: usize,
    current_row: usize,
    current_col: usize,
    sudoku: &'a Sudoku,
    done: bool,
}

impl<'a> SudokuSecIter<'a> {
    pub fn new(start_row: usize, start_col: usize, sudoku: &'a Sudoku) -> Self {
        Self {
            start_row,
            start_col,
            current_row: start_row,
            current_col: start_col,
            sudoku,
            done: false,
        }
    }
}

impl Iterator for SudokuSecIter<'_> {
    type Item = Option<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let col_before = self.current_col;
        let row_before = self.current_row;
        if self.current_col == self.start_col + self.sudoku.sec_width() - 1 {
            if self.current_row == self.start_row + self.sudoku.sec_height() - 1 {
                self.done = true;
                Some(self.sudoku.get(col_before, row_before))
            } else {
                self.current_col = self.start_col;
                self.current_row += 1;
                Some(self.sudoku.get(col_before, row_before))
            }
        } else {
            self.current_col += 1;
            Some(self.sudoku.get(col_before, row_before))
        }
    }
}
