use std::{fs, cmp::{max, min}};

struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    fn from_line(line: &str) -> Self {
        let s: Vec<&str> = line.split(" -> ").collect();
        let start_str: Vec<&str>  = s[0].split(",").collect();
        let end_str: Vec<&str>  = s[1].split(",").collect();

        Self {
            start: (start_str[0].parse().unwrap(), start_str[1].parse().unwrap()),
            end: (end_str[0].parse().unwrap(), end_str[1].parse().unwrap()),
        }
    }

    fn max_data(&self) -> (usize, usize) {
        let x = max(self.start.0, self.end.0);
        let y = max(self.start.1, self.end.1);
        (x, y)        
    }

    fn max_from_lines(lines: &Vec<Line>) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;

        for line in lines {
            let (l_x, l_y) = line.max_data();
            max_x = max(max_x, l_x);
            max_y = max(max_y, l_y);
        }
        (max_x, max_y)
    }

    fn get_cells(&self) -> Vec<(usize, usize)> {
        let mut cells: Vec<(usize, usize)> = Vec::new();

        // it's a row
        if self.start.0 == self.end.0 {
            let up = max(self.start.1, self.end.1);
            let down = min(self.start.1, self.end.1);

            for j in down..=up {
                cells.push((self.start.0, j));
            }
        }
        // it's a col
        else if self.start.1 == self.end.1 {
            let up = max(self.start.0, self.end.0);
            let down = min(self.start.0, self.end.0);

            for i in down..=up  {
                cells.push((i, self.start.1));
            }
        }
        // it's a diag
        else {
            let mut cur = (self.start.0, self.start.1);
            cells.push(cur.clone());

            while cur != self.end {
                if cur.0 > self.end.0 {
                    cur.0 -= 1;
                } else if cur.0 < self.end.0 {
                    cur.0 += 1;
                }

                if cur.1 > self.end.1 {
                    cur.1 -= 1;
                } else if cur.1 < self.end.1 {
                    cur.1 += 1;
                }
                cells.push(cur.clone());
            }
        }

        cells
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("[-] Cannot open 'input.txt' file");

    let mut lines: Vec<Line> = Vec::new();
    for ld in data.split("\n") {
        if ld.ne("") {
            lines.push(Line::from_line(ld));
        }
    }

    let board_size: (usize, usize) = Line::max_from_lines(&lines);
    let mut board: Vec<Vec<usize>> = vec![vec![0; board_size.1 + 1]; board_size.0 + 1];

    for line in lines {
        let cells = line.get_cells();
        println!("{:?} -> {:?} ==> {:?}", line.start, line.end, cells);
        for (cx, cy) in cells {
            board[cy][cx] += 1;
        }
    }

    // counting cells > 2
    let nb_cells = board.into_iter().flatten().filter(|cell| cell > &1).count();

    println!("Nb cells : {}", nb_cells);
}
