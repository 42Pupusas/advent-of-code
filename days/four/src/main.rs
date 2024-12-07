fn main() {
    let scanner_grid = ScannerGrid::new();
    let sections = scanner_grid.scan_sections();
    for row in scanner_grid.rows.iter() {
        println!("{:?}", row);
    }
    sections.iter().for_each(|section| {
        println!();
        for row in section.section.iter() {
            println!("{:?}", row);
        }
    });
    println!("XMAS Counter: {}", sections.len());
}
pub const TEST_STR_2: &str = r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"#;
pub const XMASCROSS: [char; 3] = ['M', 'A', 'S'];
pub const REV_XMASCROSS: [char; 3] = ['S', 'A', 'M'];

#[derive(Debug)]
pub struct GridSections {
    pub section: [[char; 3]; 3],
}
impl GridSections {
    pub fn validate_section(&self) -> bool {
        let diag_one = [self.section[0][0], self.section[1][1], self.section[2][2]];

        let diag_two = [self.section[0][2], self.section[1][1], self.section[2][0]];
        if diag_one == XMASCROSS || diag_one == REV_XMASCROSS {
            if diag_two == XMASCROSS || diag_two == REV_XMASCROSS {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
pub struct ScannerGrid {
    pub xmas_counter: u32,
    pub rows: Vec<Vec<char>>,
}
impl ScannerGrid {
    pub fn scan_sections(&self) -> Vec<GridSections> {
        let mut sections = vec![];
        for i in 0..self.rows.len() - 2 {
            if i + 2 >= self.rows.len() {
                continue;
            }
            for j in 0..self.rows.len() - 2 {
                if j + 2 >= self.rows.len() {
                    continue;
                }
                let mut section = [[0 as char; 3]; 3];
                for k in 0..3 {
                    for l in 0..3 {
                        if i + k >= self.rows.len() - 1 || j + l >= self.rows.len() - 1 {
                            continue;
                        }
                        section[k][l] = self.rows[i + k][j + l];
                    }
                }
                let new_section = GridSections { section };
                if new_section.validate_section() {
                    sections.push(new_section);
                }
            }
        }
        sections
    }
    pub fn new_test() -> Self {
        let grid_text = TEST_STR_2;
        let rows = grid_text
            .split("\n")
            .filter_map(|x| {
                let row = x.chars().collect::<Vec<char>>();
                if !row.is_empty() {
                    Some(row)
                } else {
                    None
                }
            })
            .collect();
        Self {
            xmas_counter: 0,
            rows,
        }
    }
    pub fn new() -> Self {
        let rows = std::fs::read_to_string("days/four/input.txt")
            .unwrap()
            .split("\n")
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        Self {
            xmas_counter: 0,
            rows,
        }
    }
}

pub const TEST_STR: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

type XmasType = [char; 4];
const XMAS: XmasType = ['X', 'M', 'A', 'S'];
const REVERSE_XMAS: XmasType = ['S', 'A', 'M', 'X'];

fn part_one() {
    let input_txt = std::fs::read_to_string("days/four/input.txt").unwrap();
    let grid_text = input_txt.trim().split("\n").collect::<Vec<&str>>();
    let mut empty_grid = Grid::new();
    grid_text.iter().for_each(|x| {
        let new_char = x.chars().collect::<Vec<char>>();
        empty_grid.rows.push(new_char.clone());
    });
    empty_grid.build_columns_from_rows();
    empty_grid.build_diagonal_slices();
    empty_grid.build_opposite_diagonal_slices();
    empty_grid.find_row_xmas();
    empty_grid.find_col_xmas();
    empty_grid.find_diagonal_xmas();
    println!("XMAS Counter: {}", empty_grid.xmas_counter);
}

#[derive(Debug)]
pub struct Grid {
    pub xmas_counter: u32,
    pub rows: Vec<Vec<char>>,
    pub cols: Vec<Vec<char>>,
    pub diagonals: Vec<Vec<char>>,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            xmas_counter: 0,
            rows: vec![],
            cols: vec![],
            diagonals: vec![],
        }
    }
    pub fn build_columns_from_rows(&mut self) {
        let mut cols = vec![];
        for i in 0..self.rows.len() {
            let mut col = vec![];
            for j in 0..self.rows.len() {
                col.push(self.rows[j][i]);
            }
            cols.push(col);
        }
        self.cols = cols;
    }
    pub fn build_diagonal_slices(&mut self) {
        let mut slices = vec![];
        for i in 0..self.rows.len() {
            let mut slice = vec![];
            for j in 0..self.rows.len() {
                if i + j < self.rows.len() {
                    slice.push(self.rows[i + j][j]);
                }
            }
            slices.push(slice);
        }
        for i in 1..self.rows.len() {
            let mut slice = vec![];
            for j in 0..self.rows.len() {
                if i + j < self.rows.len() {
                    slice.push(self.rows[j][i + j]);
                }
            }
            slices.push(slice);
        }
        self.diagonals.extend(slices);
    }
    pub fn build_opposite_diagonal_slices(&mut self) {
        let mut slices = vec![];
        for i in 0..self.rows.len() {
            let mut slice = vec![];
            for j in 0..self.rows.len() {
                if i + j < self.rows.len() {
                    slice.push(self.rows[i + j][self.rows.len() - j - 1]);
                }
            }
            slices.push(slice);
        }
        for i in 1..self.rows.len() {
            let mut slice = vec![];
            for j in 0..self.rows.len() {
                if i + j < self.rows.len() {
                    slice.push(self.rows[j][self.rows.len() - i - j - 1]);
                }
            }
            slices.push(slice);
        }
        self.diagonals.extend(slices);
    }
    pub fn find_row_xmas(&mut self) {
        self.rows.iter().enumerate().for_each(|(row_num, row)| {
            row.windows(4).enumerate().for_each(|(window_pos, window)| {
                if window == XMAS || window == REVERSE_XMAS {
                    println!("Found ROW XMAS at row: {}, col: {}", row_num, window_pos);
                    self.xmas_counter += 1;
                }
            });
        });
    }
    pub fn find_col_xmas(&mut self) {
        self.cols.iter().enumerate().for_each(|(col_num, col)| {
            // println!("COL: {:?}", col);
            col.windows(4).enumerate().for_each(|(row_num, x)| {
                // `println!("Window: {:?}", x);
                if x == XMAS || x == REVERSE_XMAS {
                    println!("Found COL XMAS at row: {}, col: {}", row_num, col_num);
                    self.xmas_counter += 1;
                }
            });
        });
    }
    pub fn find_diagonal_xmas(&mut self) {
        self.diagonals
            .iter()
            .enumerate()
            .for_each(|(col_num, col)| {
                // println!("COL: {:?}", col);
                col.windows(4).enumerate().for_each(|(row_num, x)| {
                    // `println!("Window: {:?}", x);
                    if x == XMAS || x == REVERSE_XMAS {
                        println!("Found DIAGONAL XMAS at row: {}, col: {}", row_num, col_num);
                        self.xmas_counter += 1;
                    }
                });
            });
    }
    pub fn print_rows(&self) {
        let col_nums: Vec<char> = (0..self.rows[0].len() as u32)
            .map(|x| std::char::from_digit(x, 10).unwrap())
            .collect();
        println!("C {:?}", col_nums);
        self.rows.iter().enumerate().for_each(|(i, row)| {
            println!("{} {:?}", i, row);
        });
    }
    pub fn print_cols(&self) {
        let col_nums: Vec<u32> = (0..self.cols.len() as u32).collect();
        println!("{:?}", col_nums);
        for col in &self.cols {
            println!("{:?}", col);
        }
    }
    pub fn print_diagonals(&self) {
        let col_nums: Vec<u32> = (0..self.diagonals.len() as u32).collect();
        println!("{:?}", col_nums);
        for col in &self.diagonals {
            println!("{:?}", col);
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn text_to_grid() {
        let grid_text = super::TEST_STR;
        let grid_text = grid_text.trim().split("\n").collect::<Vec<&str>>();
        let mut empty_grid = super::Grid::new();
        grid_text.iter().for_each(|x| {
            let new_char = x.chars().collect::<Vec<char>>();
            empty_grid.rows.push(new_char.clone());
        });
        empty_grid.build_columns_from_rows();
        empty_grid.build_diagonal_slices();
        empty_grid.build_opposite_diagonal_slices();
        println!();
        println!("Rows");
        empty_grid.print_rows();
        println!();
        println!("Cols");
        empty_grid.print_cols();
        println!("Diagonals");
        empty_grid.print_diagonals();

        empty_grid.find_row_xmas();
        empty_grid.find_col_xmas();
        empty_grid.find_diagonal_xmas();

        assert_eq!(empty_grid.xmas_counter, 18);
    }
}
