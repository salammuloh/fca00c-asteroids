use crate::{
    storage::{get_range, get_seed},
    types::{DataKey, Direction, Error, MapElement, Point},
};
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use soroban_sdk::{panic_with_error, Env, Map, Vec};

fn calc(e: &Env, x: i32) -> i32 {
    let range = get_range(e) as i32;
    let div = x / range;
    let rem = x % range;

    if div > rem {
        (range + 1) * (div - 1) + range / 2
    } else {
        (range + 1) * (div) + range / 2
    }
}

pub fn calc_center(e: &Env, point: Point) -> Point {
    Point(calc(e, point.0), calc(e, point.1))
}

fn access_rng(rng: &mut SmallRng, range_center: Point, range: i32) -> (i32, i32) {
    let x = rng.gen_range((range_center.0 - range / 2)..=(range_center.0 + range / 2));
    let y = rng.gen_range((range_center.1 - range / 2)..=(range_center.1 + range / 2));

    (x, y)
}

pub fn build_range_map(e: &Env, range_center: Point) -> Map<Point, MapElement> {
    let mut map = Map::new(e);
    let fixed_seed = get_seed(e);
    let mut rng =
        SmallRng::seed_from_u64(((range_center.0 * fixed_seed as i32) + range_center.1) as u64);

    let asteroid_range_density: u32 = e.storage().get(&DataKey::AstDensity).unwrap().unwrap(); // 6 asteroids in the 16x16 galaxy grid
    let fuel_pod_range_density: u32 = e.storage().get(&DataKey::PodDensity).unwrap().unwrap(); // 2 fuel pods in the 16x16 galaxy grid

    let range = get_range(e) as i32;

    for _ in 0..asteroid_range_density {
        let g = access_rng(&mut rng, range_center, range);
        let point = Point(g.0, g.1);

        if !e.storage().has(&DataKey::Expired(point)) {
            map.set(point, MapElement::Asteroid)
        }
    }

    for _ in 0..fuel_pod_range_density {
        let g = access_rng(&mut rng, range_center, range);
        let point = Point(g.0, g.1);

        if !e.storage().has(&DataKey::Expired(point)) {
            map.set(point, MapElement::FuelPod)
        }
    }

    map
}

pub fn get_laser_collisions(
    e: &Env,
    user_position: Point,
    direction: Direction,
    range: i32,
) -> Vec<Point> {
    let mut collisions: Vec<Point> = Vec::new(e);
    match direction as u32 {
        0 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0, user_position.1 + n))
            }
        }
        1 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1 + n))
            }
        }
        2 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1))
            }
        }
        3 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1 - n))
            }
        }
        4 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0, user_position.1 - n))
            }
        }
        5 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1 - n))
            }
        }
        6 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1))
            }
        }
        7 => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1 + n))
            }
        }

        _ => {
            panic_with_error!(e, Error::UnknownErr);
        }
    };

    collisions
}
