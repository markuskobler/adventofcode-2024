fn main() {
    let input = include_str!("../../data/day04.txt");

    let part1 = process_board(input);
    eprintln!("Part 1: {part1}");
}

fn process_board(board: &str) -> usize {
    let width = board.find("\n").unwrap();
    let mut count = 0;

    for (idx, ch) in board.chars().into_iter().enumerate() {
        if 'X' == ch {
            if find_xmas(board, width, idx, -1, 0) {
                count += 1;
            }
            if find_xmas(board, width, idx, -1, -1) {
                count += 1;
            }
            if find_xmas(board, width, idx, 0, -1) {
                count += 1;
            }
            if find_xmas(board, width, idx, 1, -1) {
                count += 1;
            }
            if find_xmas(board, width, idx, 1, 0) {
                count += 1;
            }
            if find_xmas(board, width, idx, 1, 1) {
                count += 1;
            }
            if find_xmas(board, width, idx, 0, 1) {
                count += 1;
            }
            if find_xmas(board, width, idx, -1, 1) {
                count += 1;
            }
        }
    }

    count
}

fn find_xmas(board: &str, width: usize, offset: usize, x: isize, y: isize) -> bool {
    let board = board.as_bytes();
    let mut o = offset as isize;

    let max_x = (o % (width as isize + 1)) + (x * 3);

    if max_x < 0 || max_x > width as isize {
        return false;
    }

    let max_y = o + (y * 3 * (width as isize + 1)) + (x * 3);
    if max_y < 0 || max_y > board.len() as isize {
        return false;
    }

    let add = (y * (width + 1) as isize) + x;

    o += add;
    if b'M' != board[o as usize] {
        return false;
    }
    o += add;
    if b'A' != board[o as usize] {
        return false;
    }
    o += add;
    if b'S' != board[o as usize] {
        return false;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input() {
        let board = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(process_board(board), 18);
    }
}
