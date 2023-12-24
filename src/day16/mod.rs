use std::collections::HashSet;

use crate::get_input;

enum Type {
    Splitter,
    Empty,
    Mirror,
}

struct Tile {
    t: Type,
    dir1: (i32, i32),
    dir2: (i32, i32),
}

pub fn solve() -> (usize, usize) {
    let input = get_input!("16");

    let mut map = Vec::new();
    for line in input {
        if let Ok(line) = line {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '/' => row.push(Tile {
                        t: Type::Mirror,
                        dir1: (1, -1),
                        dir2: (-1, 1),
                    }),
                    '\\' => row.push(Tile {
                        t: Type::Mirror,
                        dir1: (1, 1),
                        dir2: (-1, -1),
                    }),
                    '.' => row.push(Tile {
                        t: Type::Empty,
                        dir1: (0, 0),
                        dir2: (0, 0),
                    }),
                    '-' => row.push(Tile {
                        t: Type::Splitter,
                        dir1: (0, 1),
                        dir2: (0, -1),
                    }),
                    '|' => row.push(Tile {
                        t: Type::Splitter,
                        dir1: (1, 0),
                        dir2: (-1, 0),
                    }),
                    _ => (),
                }
            }
            map.push(row);
        }
    }

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut max = 0;
    let mut first = 0;
    for dir in dirs {
        for x in 0..map.len() {
            let mut energized = HashSet::new();
            let start;
            if dir.0 == 0 {
                if dir.1 == 1 {
                    start = (x as i32, 0);
                } else {
                    start = (x as i32, map[0].len() as i32 - 1);
                }
            } else {
                if dir.0 == 1 {
                    start = (0, x as i32);
                } else {
                    start = (map.len() as i32 - 1, x as i32);
                }
            }
            let mut beams = vec![(start, dir)];
            while beams.len() > 0 {
                for i in (0..beams.len()).rev() {
                    let mut beam = beams[i];
                    let tile = &map[beam.0 .0 as usize][beam.0 .1 as usize];
                    match tile.t {
                        Type::Empty => {}
                        Type::Mirror => {
                            if beam.1 .0 == tile.dir1.0 {
                                beam.1 .1 = tile.dir1.1;
                                beam.1 .0 = 0;
                            } else if beam.1 .0 == tile.dir2.0 {
                                beam.1 .1 = tile.dir2.1;
                                beam.1 .0 = 0;
                            } else if beam.1 .1 == tile.dir1.1 {
                                beam.1 .0 = tile.dir1.0;
                                beam.1 .1 = 0;
                            } else if beam.1 .1 == tile.dir2.1 {
                                beam.1 .0 = tile.dir2.0;
                                beam.1 .1 = 0;
                            }
                        }
                        Type::Splitter => {
                            let pos = (beam.0 .0, beam.0 .1);
                            let dirs;
                            if beam.1 .0 == 0 {
                                dirs = ((tile.dir1.0, 0), (tile.dir2.0, 0));
                            } else {
                                dirs = ((0, tile.dir1.1), (0, tile.dir2.1));
                            }

                            if dirs != ((0, 0), (0, 0)) {
                                beams.push((pos, dirs.0));
                                beams.push((pos, dirs.1));

                                beams.remove(i);
                                continue;
                            }
                        }
                    }

                    if energized.contains(&beam) {
                        beams.remove(i);
                        continue;
                    } else {
                        energized.insert(beam);
                    }

                    beam.0 .0 += beam.1 .0;
                    beam.0 .1 += beam.1 .1;

                    if beam.0 .0 < 0
                        || beam.0 .0 as usize >= map.len()
                        || beam.0 .1 < 0
                        || beam.0 .1 as usize >= map[0].len()
                    {
                        beams.remove(i);
                        continue;
                    }

                    beams[i] = beam;
                }
            }

            let mut energized_tiles = vec![vec![false; map[0].len()]; map.len()];
            for tile in energized {
                energized_tiles[tile.0 .0 as usize][tile.0 .1 as usize] = true;
            }

            let mut count = 0;
            for row in energized_tiles {
                for tile in row {
                    if tile {
                        count += 1;
                    }
                }
            }
            max = max.max(count);
            if first != 0 {
                continue;
            }
            first = count;
        }
    }

    return (first, max);
}
