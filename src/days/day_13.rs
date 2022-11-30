use std::cmp::Ordering;
use Entry::*;
use Ordering::*;
use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Entry {
    List(Vec<Entry>),
    Int(i32)
}

fn parse_iter(iter: &mut std::slice::Iter<&str>) -> Option<Entry>{
    match iter.next() {
        None => None,
        Some(str) => match str {
            &"[" => {
                let mut vec = vec!();
                while let Some(entry) = parse_iter(iter) {
                    vec.push(entry);
                }
                Some(Entry::List(vec))
            },
            &"]" => None,
            _ => Some(Entry::Int(str.parse().unwrap()))
        }
    }
}

fn parse_str(line: &str) -> Entry {
    let a = line.replace(",", " ").replace("[", " [ ").replace("]", " ] ");
    let b = a.split_whitespace().into_iter().collect_vec();
    parse_iter(&mut b.iter()).unwrap()
}

fn cmp(left: &Entry, right: &Entry) -> Ordering {
    match (left, right) {
        (Int(a), Int(b)) => a.cmp(b),
        (Int(a), List(_)) => cmp(&List(vec![Int(*a)]), right),
        (List(_), Int(b)) => cmp(left, &List(vec![Int(*b)])),
        (List(a), List(b)) => {
            for i in 0..a.len() {
                if i >= b.len() {
                    return Greater;
                }
                match cmp(&a[i], &b[i]) {
                    Equal => {},
                    r => return r
                }
            }
            return Less;
        }
    }
}

pub fn run_part(_input: &String, _part: u8) {
    let mut all_entries = _input.lines()
        .filter(|l| l.len() != 0)
        .map(|m| parse_str(m))
        .collect_vec();

    if _part == 1 {
        let res = all_entries
            .iter()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_vec())
            .enumerate()
            .filter(|(_, chunk)| cmp(chunk[0], chunk[1]).is_lt())
            .map(|(i, _)| i + 1)
            .sum::<usize>();
            dbg!(res);
        } else {
            let first_packet = parse_str("[[2]]");
            let second_packet = parse_str("[[6]]");

            all_entries.push(first_packet.clone());
            all_entries.push(second_packet.clone());
            all_entries.sort_by(cmp);

            let res = all_entries
                .iter()
                .enumerate()
                .filter(|(_, packet)| *packet == &first_packet || *packet == &second_packet)
                .map(|(i, _)| i + 1)
                .product::<usize>();

            dbg!(res);
        }

}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}

#[cfg(test)]
mod day13_tests {
    use super::*;
    
    #[test]
    fn day13_1() {
        // [1,1,3,1,1]
        let a = Entry::List(vec![
            Entry::Int(1),
            Entry::Int(1),
            Entry::Int(3),
            Entry::Int(1),
            Entry::Int(1),
            ]);        
        // [1,1,5,1,1]    
        let b = Entry::List(vec![
            Entry::Int(1),
            Entry::Int(1),
            Entry::Int(5),
            Entry::Int(1),
            Entry::Int(1),
        ]);
        assert_eq!(cmp(&a, &b), Less);
        
    }

    #[test]
    fn day13_2() {
        // [[1],[2,3,4]]
        let a = Entry::List(vec![
            Entry::List(vec![Entry::Int(1)]),
            Entry::List(vec![
                Entry::Int(2),
                Entry::Int(3),
                Entry::Int(4),
                ])
            ]);
        // [[1],4]
        let b = Entry::List(vec![
            Entry::List(vec!(Entry::Int(1))),
            Entry::Int(4)
        ]);
        assert_eq!(cmp(&a, &b), Less);
    }

    #[test]
    fn day13_3() {
        // [9] vs [[8,7,6]]
        let a = Entry::List(vec!(Entry::Int(9)));
        let b = List(vec![List(vec![Int(8),Int(7),Int(6)])]);
        assert_eq!(cmp(&a, &b), Greater);
    }

    #[test]
    fn day13_4() {
        let entry = parse_str(&"[2]".to_owned());
        assert_eq!(entry, Entry::List(vec!(Entry::Int(2))));
    }

    #[test]
    fn day13_5() {
        let entry = parse_str(&"[[8,7,6]]".to_owned());
        assert_eq!(entry, List(vec![List(vec![Int(8),Int(7),Int(6)])]));
    }

    #[test]
    fn day13_6() {
        let entry = parse_str(&"[[1],[2,3,4]]".to_owned());
        assert_eq!(entry, Entry::List(vec![
            Entry::List(vec![Entry::Int(1)]),
            Entry::List(vec![
                Entry::Int(2),
                Entry::Int(3),
                Entry::Int(4),
                ])
            ]));
    }

}