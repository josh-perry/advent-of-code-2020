use std::fs;

struct Map {
    tiles: Vec<bool>,
    width: i32,
    height: i32
}

struct Toboggan {
    x: i32,
    y: i32,
    trees_passed: i32
}

fn read_map(filename : String) -> Map {
    let mut found_newline = false;
    let mut map = Map {
        tiles: vec![],
        width: 0,
        height: 1
    };

    let contents = fs::read_to_string(filename).expect("Failed to read file!");

    for c in contents.chars() {
        if c == '\n' {
            found_newline = true;
            map.height += 1;
            continue;
        }

        if !found_newline {
            map.width += 1;
        }

        map.tiles.push(c == '#');
    }

    return map;
}

fn get_if_tree(map: &Map, x: usize, y: usize) -> bool {
    return map.tiles[((map.width as usize)*y) + x];
}

fn slope(map: &Map, slope_x: i32, slope_y: i32) -> i32 {
    let mut toboggan = Toboggan {
        x: 0,
        y: 0,
        trees_passed: 0
    };

    loop {
        if get_if_tree(&map, (toboggan.x % map.width) as usize, toboggan.y as usize) {
            toboggan.trees_passed += 1;
        }

        toboggan.x += slope_x;
        toboggan.y += slope_y;

        if toboggan.y >= map.height - 1 {
            break;
        }
    }

    return toboggan.trees_passed;
}

pub fn part_1(filename: String) -> i32 {
    let result = slope(&read_map(filename), 3, 1);

    println!("Passed {} trees", result);
    return result;
}

pub fn part_2(filename: String) -> i32 {
    let map = read_map(filename);
    let result = slope(&map, 1, 1) * slope(&map, 3, 1) * slope(&map, 5, 1) * slope(&map, 7, 1) * slope(&map, 1, 2);

    println!("Passed {} trees", result);
    return result;
}
