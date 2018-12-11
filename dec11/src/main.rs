fn main() {
    let mut grid = Grid::new(300, 300);
    grid.set_power(8979);
    let mut mmax_x = 0;
    let mut mmax_y = 0;
    let mut mmax_val = 0;
    let mut mmax_size = 0;
    // This is really slow (like cubic) because I do a dumb cumsum
    // and I also don't figure out the maximum possible value of size
    for size in 1..301 {
        let new_grid = grid.cumsum_rows(size).cumsum_cols(size);
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_val = -10;
        for x in 0..new_grid.width {
            for y in 0..new_grid.height {
                if new_grid.values[y][x] > max_val {
                    max_x = x + 1;
                    max_y = y + 1;
                    max_val = new_grid.values[y][x];
                }
            }
        }
        if size == 3 {
            println!("The answer for size 3 is {}, {}: {}", max_x, max_y, max_val);
        }
        if max_val > mmax_val {
            println!("New max {} {} {} {}", size, max_x, max_y, max_val);
            mmax_x = max_x;
            mmax_y = max_y;
            mmax_val = max_val;
            mmax_size = size;
        }
    }
    println!("The answer for all sizes is size {} at {}, {}: {}", mmax_size, mmax_x, mmax_y, mmax_val);
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<Vec<i64>>
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let mut values: Vec<Vec<i64>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<i64> = Vec::new();
            for x in 0..width {
                row.push(0);
            }
            values.push(row);
        }
        Grid { width: width, height: height, values: values }
    }

    fn set_power(&mut self, serial_number: i64) {
        for x in 0..self.width {
            for y in 0..self.height {
                let rack_id: i64 = (x + 10 + 1) as i64;
                let power_base = rack_id * (y + 1) as i64;
                let power_plus = power_base + serial_number;
                let power_level = power_plus * rack_id;
                let power_almost = (power_level % 1000) / 100;
                let power = power_almost - 5;
                assert!(power < 10 && power > -10);
                self.values[y][x] = power;
            }
        }
    }

    fn cumsum_rows(&self, window: usize) -> Grid {
        let new_width = self.width - window + 1;
        let new_height = self.height;
        let mut new_values: Vec<Vec<i64>> = Vec::new();
        for row in self.values.iter() {
            let mut new_row = Vec::new();
            for x in 0..(row.len() - window + 1) {
                let total = row.iter().skip(x).take(window).sum();
                new_row.push(total);
            }
            new_values.push(new_row);
        }
        Grid { width: new_width, height: new_height, values: new_values }
    }

    fn cumsum_cols(&self, window: usize) -> Grid {
        let mut new_grid = Grid::new(self.width, self.height - window + 1);
        for x in 0..new_grid.width {
            for y in 0..new_grid.height {
                let mut total = 0;
                for i in 0..window {
                    total += self.values[y + i][x];
                }
                new_grid.values[y][x] = total;
            }
        }
        new_grid
    }
}