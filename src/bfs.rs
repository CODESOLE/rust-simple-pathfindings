use std::collections::VecDeque;

fn visualize(map: &[[i32; 10]; 10], visited_nodes: &[[Option<(i32, i32)>; 10]; 10], target: i32) {
    for y in 0..visited_nodes.len() {
        for x in 0..visited_nodes.len() {
            if map[y][x] == target && visited_nodes[y][x].is_some() {
                print!("X ");
            } else if visited_nodes[y][x].is_some() {
                print!("@ ");
            } else if map[y][x] == 0 {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn bfs(map: [[i32; 10]; 10], start: (i32, i32), target: i32) {
    let mut visited_nodes = [[None; 10]; 10];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((start.1, start.0));
    let mut iteration = 0;

    // Put some filler into the first location
    visited_nodes[start.1 as usize][start.0 as usize] = Some(start);

    while let Some((y, x)) = queue.pop_front() {
        iteration += 1;
        println!("\nIteration {})", iteration);
        visualize(&map, &visited_nodes, target);

        // Check if this position is the target
        if map[y as usize][x as usize] == target {
            let mut walked_nodes = Vec::new();
            walked_nodes.push((y, x));

            let mut prev_x = x;
            let mut prev_y = y;
            while prev_x != 0 || prev_y != 0 {
                let (py, px) = visited_nodes[prev_y as usize][prev_x as usize].unwrap();
                walked_nodes.push((py, px));
                prev_y = py;
                prev_x = px;
            }

            return;
        }

        // Iterate over adjacent offsets
        for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // Check if offset is within bounds
            if x + dx < 0
                || y + dy < 0
                || (y + dy) as usize >= map.len()
                || (x + dx) as usize >= map[(y + dy) as usize].len()
            {
                continue;
            }

            // Check if offset points to valid location
            if map[(y + dy) as usize][(x + dx) as usize] == 0 {
                continue;
            }

            if visited_nodes[(y + dy) as usize][(x + dx) as usize].is_some() {
                continue;
            }

            visited_nodes[(y + dy) as usize][(x + dx) as usize] = Some((y, x));
            queue.push_back((y + dy, x + dx));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs() {
        let data = [
            [1, 0, 1, 1, 1, 1, 1, 1, 1, 2],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0],
            [1, 1, 1, 1, 0, 1, 0, 1, 1, 1],
            [0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
            [1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 1, 0, 1, 1, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        bfs(data, (2, 0), 2);
    }
}
