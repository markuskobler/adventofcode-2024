use adventofcode_2024::prelude::*;
use winnow::{
    ascii::line_ending,
    combinator::{alt, opt, repeat, separated, terminated},
    error::ParserError,
    prelude::*,
};

fn main() {
    let input = include_str!("../../data/day06.txt");

    let mut map = parse_map::<()>.parse(input).unwrap();
    let (direction, x, y) = locate_guard(&mut map);
    let result = navigate_map(&mut map, direction, x, y);

    eprintln!("Part 1: {result}");
}

type Map = Vec<Vec<Cell>>;

#[derive(PartialEq, Debug)]
enum Cell {
    Wall,
    Space(bool), // was visited
    Guard(Direction),
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_map_row<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<Vec<Cell>, E> {
    repeat(
        1..,
        alt((
            ".".map(|_| Cell::Space(false)),
            "#".map(|_| Cell::Wall),
            "^".map(|_| Cell::Guard(Direction::Up)),
            ">".map(|_| Cell::Guard(Direction::Right)),
            "v".map(|_| Cell::Guard(Direction::Down)),
            "<".map(|_| Cell::Guard(Direction::Left)),
        )),
    )
    .parse_next(input)
}

fn parse_map<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<Map, E> {
    terminated(separated(1.., parse_map_row, line_ending), opt(line_ending)).parse_next(input)
}

fn locate_guard(map: &mut Map) -> (Direction, usize, usize) {
    for (y, row) in map.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if let Cell::Guard(d) = cell {
                let direction = *d;
                *cell = Cell::Space(true);
                return (direction, x, y);
            }
        }
    }
    panic!("Guard not found")
}

fn navigate_map(map: &mut Map, mut guard: Direction, mut x: usize, mut y: usize) -> usize {
    let mut count = 1;
    loop {
        match guard {
            Direction::Up => {
                if y == 0 {
                    return count;
                }
                match map[y - 1][x] {
                    Cell::Wall => {
                        guard = Direction::Right;
                        continue;
                    }
                    Cell::Space(false) => {
                        map[y - 1][x] = Cell::Space(true);
                        count += 1;
                    }
                    Cell::Space(true) => {
                        // ignore
                    }
                    _ => unreachable!(),
                }
                y -= 1;
            }
            Direction::Right => {
                if x == map[0].len() - 1 {
                    return count;
                }
                match map[y][x + 1] {
                    Cell::Wall => {
                        guard = Direction::Down;
                        continue;
                    }
                    Cell::Space(false) => {
                        map[y][x + 1] = Cell::Space(true);
                        count += 1;
                    }
                    Cell::Space(true) => {
                        // ignore
                    }
                    _ => unreachable!(),
                }
                x += 1;
            }
            Direction::Down => {
                if y == map.len() - 1 {
                    return count;
                }
                match map[y + 1][x] {
                    Cell::Wall => {
                        guard = Direction::Left;
                        continue;
                    }
                    Cell::Space(false) => {
                        map[y + 1][x] = Cell::Space(true);
                        count += 1;
                    }
                    Cell::Space(true) => {
                        // ignore
                    }
                    _ => unreachable!(),
                }
                y += 1;
            }
            Direction::Left => {
                if x == 0 {
                    return count;
                }
                match map[y][x - 1] {
                    Cell::Wall => {
                        guard = Direction::Up;
                        continue;
                    }
                    Cell::Space(false) => {
                        map[y][x - 1] = Cell::Space(true);
                        count += 1;
                    }
                    Cell::Space(true) => {
                        // ignore
                    }
                    _ => unreachable!(),
                }
                x -= 1;
            }
        }
    }
}

fn print_map(map: &Map) {
    println!();
    for row in map.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Wall => print!("#"),
                Cell::Space(false) => print!("."),
                Cell::Space(true) => print!("X"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_example_map() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let mut map = parse_map::<()>.parse(input).unwrap();

        assert_eq!(map.len(), 10);
        assert_eq!(
            map[6],
            vec![
                Cell::Space(false),
                Cell::Wall,
                Cell::Space(false),
                Cell::Space(false),
                Cell::Guard(Direction::Up),
                Cell::Space(false),
                Cell::Space(false),
                Cell::Space(false),
                Cell::Space(false),
                Cell::Space(false)
            ]
        );

        let (direction, x, y) = locate_guard(&mut map);
        assert_eq!(direction, Direction::Up);
        assert_eq!((x, y), (4, 6));
        assert_eq!(map[6][4], Cell::Space(true));

        let result = navigate_map(&mut map, direction, x, y);

        print_map(&map);

        assert_eq!(result, 41);
    }
}
