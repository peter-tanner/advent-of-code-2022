use std::{collections::HashSet, fs};

const PATH: &str = "src/input";

fn iter_directions(p: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (p.0 + 1, p.1, p.2),
        (p.0 - 1, p.1, p.2),
        (p.0, p.1 + 1, p.2),
        (p.0, p.1 - 1, p.2),
        (p.0, p.1, p.2 + 1),
        (p.0, p.1, p.2 - 1),
    ]
}

fn main() {
    let data = fs::read_to_string(PATH).expect("Error reading file");
    assert!(data.is_ascii());

    let mut x_bounds = (i32::MAX, i32::MIN);
    let mut y_bounds = (i32::MAX, i32::MIN);
    let mut z_bounds = (i32::MAX, i32::MIN);

    let update_bounds = |bound: &mut (i32, i32), v: i32| {
        if v < bound.0 {
            bound.0 = v;
        }
        if v > bound.1 {
            bound.1 = v;
        }
    };

    let mut explored_set = HashSet::<(i32, i32, i32)>::new();
    let mut points_set = HashSet::<(i32, i32, i32)>::new();
    let mut all_points = Vec::<(i32, i32, i32)>::new();
    data.trim().split('\n').for_each(|p_str| {
        let mut point = p_str.split(',').map(|x| x.parse::<i32>().unwrap());
        let point_t = (
            point.next().unwrap(),
            point.next().unwrap(),
            point.next().unwrap(),
        );
        points_set.insert(point_t);
        all_points.push(point_t);

        update_bounds(&mut x_bounds, point_t.0);
        update_bounds(&mut y_bounds, point_t.1);
        update_bounds(&mut z_bounds, point_t.2);
    });

    // SOME AREAS MAY BE UNCONNECTED AS THEY ARE DIAGONAL.
    let mut p1 = 0;
    for point in all_points {
        p1 += explore(point, &points_set, &mut explored_set);
    }

    let mut explored_set = HashSet::<(i32, i32, i32)>::new();

    // PART 2
    // EXPAND BOUNDS BY 1 TO ALLOW FOR PLACES WHERE THE DROPLET TOUCHES THE BOUNDARY AND PREVENTS THE SEARCH FROM CONTINUING.
    x_bounds.0 -= 1;
    x_bounds.1 += 1;
    y_bounds.0 -= 1;
    y_bounds.1 += 1;
    z_bounds.0 -= 1;
    z_bounds.1 += 1;

    // DFS EMPTY SPACES
    let p2 = explore_empty(
        (x_bounds.0, y_bounds.0, z_bounds.0),
        &points_set,
        &mut explored_set,
        x_bounds,
        y_bounds,
        z_bounds,
    );

    println!("PART 1 {}", p1);
    println!("PART 2 {}", p2);
}

fn explore(
    point: (i32, i32, i32),
    points_set: &HashSet<(i32, i32, i32)>,
    explored_set: &mut HashSet<(i32, i32, i32)>,
) -> usize {
    if explored_set.contains(&point) {
        if points_set.contains(&point) {
            return 0;
        }
        return 1;
    }

    explored_set.insert(point);
    if points_set.contains(&point) {
        let mut surface_area = 0;
        for adj in iter_directions(point) {
            surface_area += explore(adj, points_set, explored_set);
        }
        return surface_area;
    } else {
        return 1;
    }
}

fn test_bound(v: i32, bound: (i32, i32)) -> bool {
    v < bound.0 || v > bound.1
}

fn explore_empty(
    point: (i32, i32, i32),
    points_set: &HashSet<(i32, i32, i32)>,
    explored_set: &mut HashSet<(i32, i32, i32)>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
) -> usize {
    if explored_set.contains(&point)
        || test_bound(point.0, x_bounds)
        || test_bound(point.1, y_bounds)
        || test_bound(point.2, z_bounds)
    {
        if points_set.contains(&point) {
            return 1;
        }
        return 0;
    }

    explored_set.insert(point);
    if !points_set.contains(&point) {
        let mut surface_area = 0;
        for adj in iter_directions(point) {
            surface_area +=
                explore_empty(adj, points_set, explored_set, x_bounds, y_bounds, z_bounds);
        }
        return surface_area;
    } else {
        return 1;
    }
}
