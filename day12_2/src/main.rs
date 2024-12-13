use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<_>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut plants = HashMap::new();
    for (r, row) in input.iter().enumerate() {
        for (c, plant) in row.iter().enumerate() {
            if !plants.contains_key(plant) {
                plants.insert(*plant, vec![HashSet::from([(r, c)])]);
                continue;
            }

            let mut up_region = None;
            let mut left_region = None;
            for i in 0..plants[plant].len() {
                let region = &plants[plant][i];
                if up_region.is_none() && r > 0 && region.contains(&(r - 1, c)) {
                    up_region = Some(i);
                }
                if left_region.is_none() && c > 0 && region.contains(&(r, c - 1)) {
                    left_region = Some(i);
                }

                if (r == 0 || up_region.is_some()) && (c == 0 || left_region.is_some()) {
                    break;
                }
            }

            if up_region.is_some() && left_region.is_some() {
                let i = up_region.unwrap();
                let j = left_region.unwrap();

                if i == j {
                    plants.get_mut(plant).unwrap()[i].insert((r, c));
                    continue;
                }

                let (i, j) = if j < i { (j, i) } else { (i, j) };
                let regions = plants.get_mut(plant).unwrap();
                regions[i].insert((r, c));
                let j_region = regions.remove(j);
                regions[i].extend(&j_region);
                continue;
            }

            if up_region.is_none() && left_region.is_none() {
                plants.get_mut(plant).unwrap().push(HashSet::from([(r, c)]));
                continue;
            }

            // unwrap_or_else so it's lazy in left_region
            let i = up_region.unwrap_or_else(|| left_region.unwrap());
            plants.get_mut(plant).unwrap()[i].insert((r, c));
        }
    }

    let mut price = 0;
    for region in plants.values().flatten() {
        let area = region.len();
        let mut edge_count = 0;
        for &(r, c) in region {
            let up = r > 0 && region.contains(&(r - 1, c));
            let down = region.contains(&(r + 1, c));
            let left = c > 0 && region.contains(&(r, c - 1));
            let right = region.contains(&(r, c + 1));
            let up_left = r > 0 && c > 0 && region.contains(&(r - 1, c - 1));
            let up_right = r > 0 && region.contains(&(r - 1, c + 1));
            let down_left = c > 0 && region.contains(&(r + 1, c - 1));
            let down_right = region.contains(&(r + 1, c + 1));

            if !up && !right || up && right && !up_right {
                edge_count += 1;
            }
            if !up && !left || up && left && !up_left {
                edge_count += 1;
            }
            if !down && !right || down && right && !down_right {
                edge_count += 1;
            }
            if !down && !left || down && left && !down_left {
                edge_count += 1;
            }
        }

        price += area * edge_count;
    }

    println!("{price}");
}
