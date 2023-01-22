pub mod dfs;

use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

const SQUARE_SIZE: f32 = 100.;
const WALKABLE: Color = WHITE;
const NOT_WALKABLE: Color = GRAY;
const VISITED: Color = GREEN;

#[derive(Default)]
struct Tile {
    pos: (f32, f32),
    id: u32,
    color: Color,
}

fn calculate_tiles(xy: &str) -> Vec<Tile> {
    let s = xy.to_string();
    let x = s.lines().next().unwrap().chars().count();
    let y = s.lines().count();
    let mut tiles = Vec::<Tile>::new();

    let padding_x = (window_conf().window_width as f32 - ((SQUARE_SIZE) * x as f32)) / 2f32;
    let padding_y = (window_conf().window_height as f32 - ((SQUARE_SIZE) * y as f32)) / 2f32;

    for id in 0..(x * y) {
        let id = id as u32;
        tiles.push(Tile {
            id,
            ..Default::default()
        });
    }

    for (i, l) in s.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => tiles[i * x + j].color = NOT_WALKABLE,
                '.' => tiles[i * x + j].color = WALKABLE,
                _ => eprintln!("Invalid path"),
            }
        }
    }

    for i in 0..y {
        for j in 0..x {
            let xx = j as f32 * SQUARE_SIZE + padding_x;
            let yy = i as f32 * SQUARE_SIZE + padding_y;
            tiles[i * x + j].pos = (xx, yy);
        }
    }

    tiles
}

fn draw_map(tiles: &Vec<Tile>, path: &Vec<u32>) {
    for tile in tiles.iter() {
        if path.contains(&tile.id) {
            draw_rectangle(
                tile.pos.0 as f32,
                tile.pos.1 as f32,
                SQUARE_SIZE as f32,
                SQUARE_SIZE as f32,
                VISITED,
            );
        } else {
            draw_rectangle(
                tile.pos.0 as f32,
                tile.pos.1 as f32,
                SQUARE_SIZE as f32,
                SQUARE_SIZE as f32,
                tile.color,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "DFS".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 400,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let m_str = "...
.#.
...";

    let m = m_str.parse::<dfs::Map>().unwrap();
    let tiles = calculate_tiles(m_str);
    let mut walked_paths = Vec::new();

    if let Some(path) = dfs::dfs(m_str, &m, 0.into(), 8.into()) {
        println!("{:?}", path);
        let mut ps = path.iter();
        loop {
            if let Some(p) = ps.next() {
                walked_paths.push(*p);
                thread::sleep(Duration::from_millis(1000));
                clear_background(BLACK);

                draw_map(&tiles, &walked_paths);

                next_frame().await
            }
        }
    } else {
        eprintln!("Error at calculating path!!!");
    }
}
