use std::{fs, collections::{BinaryHeap, VecDeque}};

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();


    let mut board: Vec<Vec<usize>> = Vec::new();
    board.push(vec![9;102]);
    for line in data.split("\n") {
        let line = line.trim();
        if line.eq("") { continue; }

        let mut row: Vec<usize> = vec![9];
        row.append(&mut line.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<usize>>());
        row.push(9);

        board.push(row);
    }
    board.push(vec![9;102]);

    // println!("{:?}", board);
    let mut risk_level_sum = 0;
    for i in 1..101 {
        for j in 1..101 {
            let val = board[i][j]; if val == 9 {continue} //quick skip

            let left = board[i][j-1]; let right = board[i][j+1];
            let up = board[i-1][j]; let down = board[i+1][j];

            if val < left && val < right && val < up && val < down {
                risk_level_sum += 1 + val as u32;
            }
        }
    }
    println!("Somme: {}", risk_level_sum);

    fn adjacent_points((i, j): (usize, usize)) -> [(usize, usize); 4] {
        [(i-1,j), (i+1,j), (i,j-1), (i,j+1)]
    }
    //BFS floodfill. One-shot, once an area is filled in, the area is modified, makes it easier and quicker to implement in this structless manner
    let mut basin_areas = BinaryHeap::new(); //Max heap
    let mut fill_queue: VecDeque<(usize, usize)> = VecDeque::new();
    for i_start in 1..100+1 {
        for j_start in 1..100+1 {
            if board[i_start][j_start] > 8 {continue} //Skip non-basin points

            let mut area: u64 = 0;
            //Do one floodfill
            fill_queue.push_back((i_start, j_start));
            board[i_start][j_start] = 9; //Set height to 9 so it isn't added multiple times

            while !fill_queue.is_empty() {
                let next = fill_queue.pop_front().unwrap(); //Queue not empty, unwrap
                for (i, j) in adjacent_points(next) {
                    if board[i][j] < 9 {
                        fill_queue.push_back((i,j)); //If adjacent point is in the basin, push to queue
                        board[i][j] = 9; //Set its height 9 so it isn't added multiple times
                    }
                }
                area += 1; //Increment area once for each position that is handled
            }

            basin_areas.push(area);
            //End of floodfill
        }
    }
    //All basins filled, the heightmap is all 9's

    let first = basin_areas.pop().unwrap();
    let second = basin_areas.pop().unwrap();
    let third = basin_areas.pop().unwrap();
    println!("Product of three largest areas: {}", first*second*third);
}
