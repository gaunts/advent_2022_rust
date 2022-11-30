use std::collections::HashSet;
use either::*;
use itertools::Itertools;

fn parse(_input: &String) -> Vec<Vec<u8>> {
    let mut res = vec!();

    for line in _input.lines() {
        let mut line_vec = vec!();
        for val in line.as_bytes() {
            line_vec.push(val - b'0');
        }
        res.push(line_vec);
    }
    res
}

// C'est degueulasse mais il est tard faites pas chier
fn get_tree_points(forest: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> i32 {
    let mut res = 0;
    let current_tree = forest[y][x];

    if x != 0 {
        for x_ in (0..x).rev() {
            res = res + 1;
            if forest[y][x_] >= current_tree {
                break;
            }
        }
    }
    let mut res_total = res;
    res = 0;

    if x != forest[0].len() {
        for x_ in x+1..forest[0].len() {
            res = res + 1;
            if forest[y][x_] >= current_tree {
                break;
            }
        }
    }

    res_total = res_total * res;
    res = 0;

    if y != 0 {
        for y in (0..y).rev() {
            res = res + 1;
            if forest[y][x] >= current_tree {
                break;
            }
        }    
    }
    res_total = res_total * res;
    res = 0;

    if y != forest.len() {
        for y in y+1..forest.len() {
            res = res + 1;
            if forest[y][x] >= current_tree {
                break;
            }
        }
    }
    res_total = res_total * res;
    res_total
}

// Could be faster by not using a hashset and stopping early when going from back but I had fun like this.
fn find_trees_in_line<'a, I>(line: I, y: usize, found_trees: &mut HashSet<(usize, usize)>, column: bool, from_back: bool) 
where
    I: DoubleEndedIterator<Item = &'a u8> + std::iter::ExactSizeIterator, {

    let mut highest_line_tree = 0;
    let iter = if from_back {
        Left(line.enumerate().rev())
    } else {
        Right(line.enumerate())
    };

    let mut first = true;
    for (x, tree) in iter {
        if tree > &highest_line_tree || first {
            highest_line_tree = *tree;
            if column {
                found_trees.insert((y, x));
            } else {
                found_trees.insert((x, y));
            }
        }
        first = false;
    }
}

pub fn run_part(_input: &String, _part: u8) {
    let forest = parse(_input);
    let mut found_trees: HashSet<(usize, usize)> = HashSet::new();

    if _part == 1 {
        for (y, tree_line) in forest.iter().enumerate() {
            find_trees_in_line(tree_line.iter(), y, &mut found_trees, false, false);
            find_trees_in_line(tree_line.iter(), y, &mut found_trees, false, true);
        }
    
        for x in 0..forest[0].len() {
            find_trees_in_line(forest.iter().map(|l| &l[x]), x, &mut found_trees, true, false);
            find_trees_in_line(forest.iter().map(|l| &l[x]), x, &mut found_trees, true, true);
        }
        dbg!(found_trees.len());
    } else {
        let all_coordinates = (0..forest[0].len()).cartesian_product(0..forest.len());
        let mut tallest = 0;
        for coords in all_coordinates {
            let score = get_tree_points(&forest, coords);
            if score > tallest {
                tallest = score;
            }
        }
        dbg!(&tallest);
    }

}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
