use crate::components::*;
use crate::constants::*;
use crate::util::seed_service::*;

pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    pub fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}

struct Dsu(Vec<u32>);

impl Dsu {
    fn new(n: usize) -> Self {
        Self((0..n as u32).collect())
    }

    fn find(&mut self, mut a: usize) -> usize {
        while self.0[a] as usize != a {
            self.0[a] = self.0[self.0[a] as usize];
            a = self.0[a] as usize;
        }
        a
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.0[ra] = rb as u32;
        true
    }
}

#[derive(Clone, Debug)]
pub struct RegionMap {
    tiles: Vec<Tile>,
}

impl RegionMap {
    pub fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= REGION_TILE_SIZE || y >= REGION_TILE_SIZE {
            return Tile::Wall;
        }
        self.tiles[(y * REGION_TILE_SIZE + x) as usize]
    }

    fn fill(&mut self, x0: i32, y0: i32, w: i32, h: i32, tile: Tile) {
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                self.tiles[(y * REGION_TILE_SIZE + x) as usize] = tile;
            }
        }
    }
}

fn origin(c: usize) -> i32 {
    1 + c as i32 * CELL_PITCH
}

fn cell(cx: usize, cy: usize) -> usize {
    cy * GRID_SIZE + cx
}

fn carve(map: &mut RegionMap, a: usize, b: usize) {
    let (ax, ay) = ((a % GRID_SIZE) as i32, (a / GRID_SIZE) as i32);
    let (bx, by) = ((b % GRID_SIZE) as i32, (b / GRID_SIZE) as i32);
    let x0 = 1 + ax.min(bx) * CELL_PITCH;
    let y0 = 1 + ay.min(by) * CELL_PITCH;
    if ax != bx {
        map.fill(x0 + CELL_FLOOR, y0, CELL_PITCH - CELL_FLOOR, CELL_FLOOR, Tile::Floor);
    } else {
        map.fill(x0, y0 + CELL_FLOOR, CELL_FLOOR, CELL_PITCH - CELL_FLOOR, Tile::Floor);
    }
}

fn generate_maze(seed: u64) -> (RegionMap, Vec<Room>) {
    let mut rng = Rng(seed);
    let mut map = RegionMap {
        tiles: vec![Tile::Wall; (REGION_TILE_SIZE * REGION_TILE_SIZE) as usize],
    };
    let mut dsu = Dsu::new(GRID_SIZE * GRID_SIZE);
    let mut taken = vec![false; GRID_SIZE * GRID_SIZE];

    // Räume als Zellblöcke
    let mut rooms: Vec<Room> = Vec::new();
    let room_count = MIN_ROOM_NUM + rng.below(MAX_ROOM_NUM - MIN_ROOM_NUM + 1);
    for _ in 0..room_count {
        let room_type = random_room_type(&mut rng);
        let (w, h) = room_type.cells();
        for _try in 0..30 {
            // mind. 1 Zelle Abstand zum Rand, damit der Eingang immer einen Nachbarn hat
            let cx = 1 + rng.below(GRID_SIZE - w - 1);
            let cy = 1 + rng.below(GRID_SIZE - h - 1);
            let free = (cy.saturating_sub(1)..(cy + h + 1).min(GRID_SIZE)).all(|y| {
                (cx.saturating_sub(1)..(cx + w + 1).min(GRID_SIZE)).all(|x| !taken[cell(x, y)])
            });
            if !free {
                continue;
            }
            for y in cy..cy + h {
                for x in cx..cx + w {
                    taken[cell(x, y)] = true;
                    dsu.union(cell(x, y), cell(cx, cy));
                }
            }
            let extra = CELL_PITCH - CELL_FLOOR;
            map.fill(
                origin(cx),
                origin(cy),
                w as i32 * CELL_PITCH - extra,
                h as i32 * CELL_PITCH - extra,
                Tile::RoomFloor
            );

            // genau ein Eingang: öffnet die Wand zwischen Raum und Nachbarzelle
            let entry = random_direction(&mut rng);
            let (a, b, entry_cords) = match entry {
                Direction::NORTH => {
                    let k = rng.below(w);
                    (
                        cell(cx + k, cy + h - 1),
                        cell(cx + k, cy + h),
                        Cords::new(origin(cx + k) + 1, origin(cy + h) - 1),
                    )
                }
                Direction::SOUTH => {
                    let k = rng.below(w);
                    (
                        cell(cx + k, cy),
                        cell(cx + k, cy - 1),
                        Cords::new(origin(cx + k) + 1, origin(cy) - 1),
                    )
                }
                Direction::EAST => {
                    let k = rng.below(h);
                    (
                        cell(cx + w - 1, cy + k),
                        cell(cx + w, cy + k),
                        Cords::new(origin(cx + w) - 1, origin(cy + k) + 1),
                    )
                }
                Direction::WEST => {
                    let k = rng.below(h);
                    (
                        cell(cx, cy + k),
                        cell(cx - 1, cy + k),
                        Cords::new(origin(cx) - 1, origin(cy + k) + 1),
                    )
                }
            };
            carve(&mut map, a, b);

            let (width, height) = room_type.dimensions();
            rooms.push(Room {
                id: RoomId(rooms.len() as i32),
                cords: Cords::new(origin(cx), origin(cy)),
                width,
                height,
                entry,
                entry_cords,
                room_type,
            });
            break;
        }
    }

    // Jede Zelle ist ein Gangblock
    for cy in 0..GRID_SIZE {
        for cx in 0..GRID_SIZE {
            map.fill(origin(cx), origin(cy), CELL_FLOOR, CELL_FLOOR, Tile::Floor);
        }
    }

    // Kanten sammeln und mischen
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for cy in 0..GRID_SIZE {
        for cx in 0..GRID_SIZE {
            if cx + 1 < GRID_SIZE {
                edges.push((cell(cx, cy), cell(cx + 1, cy)));
            }
            if cy + 1 < GRID_SIZE {
                edges.push((cell(cx, cy), cell(cx, cy + 1)));
            }
        }
    }
    for i in (1..edges.len()).rev() {
        edges.swap(i, rng.below(i + 1));
    }

    // Kruskal + zufällige Zusatzdurchbrüche (Schleifen)
    for (a, b) in edges {
        if taken[a] || taken[b] {
            continue;
        }
        if dsu.union(a, b) || rng.chance(LOOP_PERCENT) {
            carve(&mut map, a, b);
        }
    }

    (map, rooms)
}

fn random_room_type(rng: &mut Rng) -> RoomType {
    match rng.below(10) {
        0..=5 => RoomType::SMALL,
        6..=8 => RoomType::NORMAL,
        _ => RoomType::LARGE,
    }
}

fn random_direction(rng: &mut Rng) -> Direction {
    match rng.below(4) {
        0 => Direction::NORTH,
        1 => Direction::EAST,
        2 => Direction::SOUTH,
        _ => Direction::WEST,
    }
}

// Zellindizes der Ausgänge an einer gemeinsamen Kante.
// kind 0 = vertikale Kante links von Region (ex, ey), kind 1 = horizontale Kante unter Region (ex, ey)
fn edge_cells(world_seed: u64, kind: i32, ex: i32, ey: i32) -> Vec<usize> {
    let mut rng = Rng(calc_any_hash_2(calc_any_hash(world_seed, kind), ex, ey));
    let range = (MAX_REGION_SIDE_CONNECTIONS - MIN_REGION_SIDE_CONNECTIONS + 1) as usize;
    let n = MIN_REGION_SIDE_CONNECTIONS as usize + rng.below(range);
    (0..n).map(|_| rng.below(GRID_SIZE)).collect()
}

fn carve_exits(map: &mut RegionMap, world_seed: u64, rx: i32, ry: i32) {
    let west = edge_cells(world_seed, 0, rx, ry);
    let east = edge_cells(world_seed, 0, rx + 1, ry);
    let south = edge_cells(world_seed, 1, rx, ry);
    let north = edge_cells(world_seed, 1, rx, ry + 1);
    let end = origin(GRID_SIZE - 1) + CELL_FLOOR;

    for c in west {
        map.fill(0, origin(c), 1, CELL_FLOOR, Tile::Floor);
    }
    for c in east {
        map.fill(end, origin(c), REGION_TILE_SIZE - end, CELL_FLOOR, Tile::Floor);
    }
    for c in south {
        map.fill(origin(c), 0, CELL_FLOOR, 1, Tile::Floor);
    }
    for c in north {
        map.fill(origin(c), end, CELL_FLOOR, REGION_TILE_SIZE - end, Tile::Floor);
    }
}

pub fn set_region_tiles(x: i32, y: i32, region: &Region) -> Tile {
    region.map.get(x - region.cords.x, y - region.cords.y)
}

// rx, ry = Regionsindex (nicht Tile-Koordinaten)
pub fn generate_region(world_seed: u64, rx: i32, ry: i32) -> Region {
    let (mut map, mut rooms) = generate_maze(calc_region_seed(world_seed, rx, ry));
    carve_exits(&mut map, world_seed, rx, ry);

    let cords = Cords {
        x: rx * REGION_TILE_SIZE,
        y: ry * REGION_TILE_SIZE,
    };

    for room in rooms.iter_mut() {
        room.cords.x += cords.x;
        room.cords.y += cords.y;
        room.entry_cords.x += cords.x;
        room.entry_cords.y += cords.y;
    }

    Region { cords, rooms, map }
}