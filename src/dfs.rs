use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

fn visualize(map: &str, visited_nodes: &Vec<u32>, target: u32) {
    let mut s = map.to_string();
    let x = map.lines().next().unwrap().chars().count() + 1;
    for &visited_node in visited_nodes.iter() {
        let ny = visited_node as usize / x;
        let nx = visited_node as usize % (x - 1);
        if visited_node != target {
            s.replace_range((ny * x + nx)..(ny * x + nx + 1), "@");
        } else {
            s.replace_range((ny * x + nx)..(ny * x + nx + 1), "X");
        }
    }
    println!("{}", s);
    println!("");
}

pub fn dfs(m: &str, map: &Map, start: Vertex, target: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::<Vertex>::new();
    queue.push_back(start);
    let mut iteration = 0;

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());

        iteration += 1;
        println!("[Iteration {}]", iteration);
        visualize(m, &history, target.value());

        if current_vertex == target {
            return Some(history);
        }

        for neighbor in current_vertex.neighbors(map).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor);
            }
        }
    }
    None
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vertex(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Edge(u32, u32);
#[derive(Clone, Debug)]
pub struct Map {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Map {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Map { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, map: &Map) -> VecDeque<Vertex> {
        map.edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::<Vertex>::new();
        let mut e = Vec::<Edge>::new();
        let mut arr_chars: Vec<Vec<char>> = Vec::new();
        let mut is_visited_list: Vec<Vec<bool>> = Vec::new();

        for (i, l) in s.lines().enumerate() {
            arr_chars.push(Vec::new());
            is_visited_list.push(Vec::new());
            for (_, _) in l.chars().enumerate() {
                arr_chars[i].push(0.into());
                is_visited_list[i].push(false);
            }
        }
        for (i, l) in s.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                arr_chars[i][j] = c;
                is_visited_list[i][j] = false;
            }
        }

        for (i, l) in arr_chars.iter().enumerate() {
            for (j, c) in l.iter().enumerate() {
                if *c == '#' {
                    continue;
                }
                if is_visited_list[i][j] {
                    continue;
                }

                v.push(((i as usize * l.len() + j as usize) as u32).into());
                is_visited_list[i as usize][j as usize] = true;

                for (dy, dx) in &[(1i32, 0i32), (0, 1), (-1, 0), (0, -1)] {
                    let i = i as i32; // y
                    let j = j as i32; // x

                    if i + dx < 0
                        || (i + dx) as usize >= arr_chars.len()
                        || j + dy < 0
                        || (j + dy) as usize >= l.len()
                    {
                        continue;
                    }

                    if arr_chars[(i + dx) as usize][(j + dy) as usize] != '#' {
                        let e1 = (i as usize * l.len() + j as usize) as u32;
                        let e2 = ((i + dx) as usize * l.len() + (j + dy) as usize) as u32;
                        if e.iter().find(|&x| *x == (e1, e2).into()).is_none()
                            && e.iter().find(|&x| *x == (e2, e1).into()).is_none()
                        {
                            e.push((e1, e2).into());
                        }
                    }
                }
            }
        }

        Ok(Map {
            vertices: v,
            edges: e,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let m_str = "...
.#.
...";
        let m = m_str.parse::<Map>().unwrap();

        println!("{:?}", dfs(m_str, &m, 0.into(), 8.into()));
        println!("{:?}", m);
    }
}
