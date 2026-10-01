use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::util::seed_service::*;
use crate::world::region::*;
use std::collections::{VecDeque, HashMap, HashSet};

//calc manhattan distance
pub fn manhattan(a: &Cords, b: &Cords) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

//check if a tile is blocked, checking collision with other rooms input path, region bounds
pub fn is_blocked(c: Cords, rooms: &[Room], start: Cords, target: Cords, region_cords: Cords) -> bool {

    if c == start || c == target {
        return false;
    }

//    if blocked_cords.iter().any(|bc| c == *bc) { return true; }

    if c.x - 1 - PATH_PADDING <= region_cords.x || c.y - 1 - PATH_PADDING <= region_cords.y {
        return true;
    }
    if c.x + 1 + PATH_PADDING >= region_cords.x + REGION_TILE_SIZE {
        return true;
    }

    if c.y + 1 + PATH_PADDING >= region_cords.y + REGION_TILE_SIZE {
        return true;
    }

    rooms.iter().any(|r| {
        c.x >= (r.cords.x - PATH_PADDING) && c.x < (r.cords.x + r.width + PATH_PADDING) &&
        c.y >= (r.cords.y - PATH_PADDING) && c.y < (r.cords.y + r.height + PATH_PADDING)
    })

}

//returning star like cords around a cords with a padding (starsize)
pub fn get_neighnbour_cords(cords: Cords, padding: i32) -> Vec<Cords> {
        let mut neighbours: Vec<Cords> = Vec::new();
    for dx in -padding..=padding {
        for dy in -padding..=padding {
            if dx == 0 && dy == 0 { continue; }

            neighbours.push(Cords {x: cords.x + dx, y: cords.y + dy})
        }
    }
    neighbours
}

//similar to get_neighnbour_cords
//returns only the touching neighbours of a cords
pub fn get_touching_neighnbour_cords(cords: Cords) -> Vec<Cords> {
    let mut neighbours: Vec<Cords> = Vec::new();
    let i = 1;

        neighbours.push(Cords { x: cords.x, y: cords.y + i });
        neighbours.push(Cords {x: cords.x + i, y: cords.y });
        neighbours.push(Cords { x: cords.x, y: cords.y - i});
        neighbours.push(Cords { x: cords.x - i, y: cords.y});

    neighbours
}

//generate a random number with max noise range, for random pathing
pub fn calc_noise(seed: u64, cords: &Cords) -> i32 {
    (calc_any_hash_2(seed, cords.x, cords.y) % NOISE_RANGE as u64) as i32
}

//calc, how long the last turn is away (with delta x and y, and run = distance)
pub fn straight_run(path: &[Cords]) -> (i32, i32, i32) {
    let n = path.len();
    if n < 2 {
        return (0, 0, 0);
    }
    let dx = path[n - 1].x - path[n - 2].x;
    let dy = path[n - 1].y - path[n - 2].y;
    let mut run = 1;
    let mut i = n - 1;
    while i >= 2
        && path[i - 1].x - path[i - 2].x == dx
        && path[i - 1].y - path[i - 2].y == dy
    {
        run += 1;
        i -= 1;
    }
    (dx, dy, run)
}

// checking, if path is too close to itself
pub fn too_close(c: Cords, path: &[Cords]) -> bool {
    let d = 2 * PATH_PADDING + 1 + MIN_GAP;

    // Die letzten 2*d Felder gehören zur "eigenen Nachbarschaft" (Kurven, Ecken)
    // und werden nicht geprüft, sonst könnte der Weg nie abbiegen.
    let skip = (2 * d) as usize;
    if path.len() <= skip {
        return false;
    }

    path[..path.len() - skip]
        .iter()
        .any(|p| (p.x - c.x).abs() < d && (p.y - c.y).abs() < d)
}

//searching
//
//

pub fn my_search((start, target): (Cords, Cords), all_rooms: &Vec<Room>, seed: u64, region_cords: Cords, noise: bool) -> Vec<Cords> {

    let mut visited: Vec<Cords> = vec![start];          // aktueller Weg
    let mut seen: HashSet<Cords> = HashSet::new();      // alles je Besuchte, wird NIE gekürzt
    let mut way_padding: Vec<Cords> = Vec::new();
    seen.insert(start);

    // Fallback: bester Stand, falls kein Weg gefunden wird
    let mut best = visited.clone();
    let mut best_dist = manhattan(&start, &target);

    println!("find way: {:?} -> {:?}", start, target);

    let mut counter = 0;

    loop {
        let last = *visited.last().unwrap();
        if last == target {
            break;
        }

        counter += 1;
        if counter >= 100000 {
            println!("best_dist: {}, best last: {:?}", best_dist, best.last());
            visited = best;
            break;
        }

        let mut candidates: Vec<Cords> = get_touching_neighnbour_cords(last)
            .into_iter()
            .filter(|n| !seen.contains(n))
            .filter(|n| !is_blocked(*n, all_rooms, start, target, region_cords))
            .filter(|n| *n == target || !too_close(*n, &visited))
            .collect();

        // Ziel direkt nehmen, sobald es erreichbar ist
        if candidates.contains(&target) {
            visited.push(target);
            continue;
        }
            // Abbiegen verbieten, solange der gerade Abschnitt noch zu kurz ist
        let (dx, dy, run) = straight_run(&visited);
        if run > 0 && run < MIN_STRAIGHT {
            let ahead = Cords { x: last.x + dx, y: last.y + dy };
            // nur erzwingen, wenn geradeaus möglich ist und nicht vom Ziel wegführt
            if candidates.contains(&ahead) && manhattan(&ahead, &target) <= manhattan(&last, &target) {
                candidates = vec![ahead];
            }
        }

        let my_cords = candidates
            .into_iter()
            .min_by_key(|r| manhattan(r, &target) + (if noise { calc_noise(seed, r) } else { 0 }));

        match my_cords {
            Some(c) => {
                seen.insert(c);
            visited.push(c);

                let d = manhattan(&c, &target);
                if d < best_dist {
                    best_dist = d;
                    best = visited.clone();
                }
            }
            None => {
                // Sackgasse: einen Schritt zurück, seen bleibt unverändert
                visited.pop();
                if visited.is_empty() {
                    visited = best;   // alles erschöpft -> Teilweg zurückgeben
                    break;
                }
            }
        }
    }

    // Padding erst am Ende, nur für den tatsächlichen Weg
    for c in &visited {
        way_padding.append(&mut get_neighnbour_cords(*c, PATH_PADDING));
    }
    visited.append(&mut way_padding);
    visited
}
