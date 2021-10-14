pub mod solver {
    pub type Board = [[u8; 9]; 9];
    pub type CellCoordinates = (usize, usize);

    pub fn is_value_possible(board: Board, coordinates: CellCoordinates, value: u8) -> bool {
        let (row, column) = coordinates;

        for i in 0..9 {
            if board[row][i] == value || board[i][column] == value {
                return false;
            }
        }

        let x_index = row / 3 * 3;
        let y_index = column / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if board[x_index + x][y_index + y] == value {
                    return false;
                }
            }
        }

        true
    }

    fn is_board_solved(board: Board) -> bool {
        coordinates_of_next_empty_cell(board) == (10, 10)
    }

    fn coordinates_of_next_empty_cell(board: Board) -> CellCoordinates {
        for row in 0..9 {
            for column in 0..9 {
                if board[row][column] == 0 {
                    return (row, column);
                }
            }
        }
        (10, 10)
    }

    fn possible_values_for_cell(board: Board, coordinates: CellCoordinates) -> Vec<u8> {
        let (row, column) = coordinates;
        let mut result = vec![];
        for guess in 1..10 {
            if is_value_possible(board, (row, column), guess) {
                result.push(guess);
            }
        }
        result
    }

    pub fn solve(board: Board) -> Board {
        if is_board_solved(board) {
            return board;
        }

        let (row, column) = coordinates_of_next_empty_cell(board);
        let mut result = board;

        for value in possible_values_for_cell(board, (row, column)) {
            result[row][column] = value;
            result = solve(result);
            if is_board_solved(result) {
                return result;
            }
        }

        result = board;
        return result;
    }
}

#[allow(dead_code)]
fn test_board() -> solver::Board {
    [
        [5, 3, 0,   0, 7, 0,    0, 0, 0],
        [6, 0, 0,   1, 9, 5,    0, 0, 0],
        [0, 9, 8,   0, 0, 0,    0, 6, 0],

        [8, 0, 0,   0, 6, 0,    0, 0, 3],
        [4, 0, 0,   8, 0, 3,    0, 0, 1],
        [7, 0, 0,   0, 2, 0,    0, 0, 6],

        [0, 6, 0,   0, 0, 0,    2, 8, 0],
        [0, 0, 0,   4, 1, 9,    0, 0, 5],
        [0, 0, 0,   0, 8, 0,    0, 7, 9],
    ]
}

#[test]
fn not_valid_1_1_5() {
    assert!(!solver::is_value_possible(test_board(), (1, 1), 5))
}

#[test]
fn not_valid_3_3_3() {
    assert!(!solver::is_value_possible(test_board(), (3, 3), 3))
}

#[test]
fn valid_3_3_5() {
    assert!(solver::is_value_possible(test_board(), (3, 3), 5))
}

#[test]
fn not_valid_4_4_3() {
    assert!(!solver::is_value_possible(test_board(), (4, 4), 3))
}

#[test]
fn valid_4_4_5() {
    assert!(solver::is_value_possible(test_board(), (4, 4), 5))
}

#[test]
fn solves_board() {
    assert_eq!(
        solver::solve(test_board()),
        [
            [5, 3, 4,   6, 7, 8,    9, 1, 2],
            [6, 7, 2,   1, 9, 5,    3, 4, 8],
            [1, 9, 8,   3, 4, 2,    5, 6, 7],

            [8, 5, 9,   7, 6, 1,    4, 2, 3],
            [4, 2, 6,   8, 5, 3,    7, 9, 1],
            [7, 1, 3,   9, 2, 4,    8, 5, 6],
            
            [9, 6, 1,   5, 3, 7,    2, 8, 4],
            [2, 8, 7,   4, 1, 9,    6, 3, 5],
            [3, 4, 5,   2, 8, 6,    1, 7, 9],
        ]
    )
}