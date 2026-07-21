impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for i in 0..m {
            for j in 0..n {
                let mut live_neighbors = 0;

                for &(dx, dy) in &directions {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;

                    if ni >= 0
                        && ni < m as i32
                        && nj >= 0
                        && nj < n as i32
                    {
                        let val = board[ni as usize][nj as usize];
                        if val == 1 || val == 2 {
                            live_neighbors += 1;
                        }
                    }
                }

                if board[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        board[i][j] = 2; // live -> dead
                    }
                } else {
                    if live_neighbors == 3 {
                        board[i][j] = 3; // dead -> live
                    }
                }
            }
        }

        // Finalize the board
        for i in 0..m {
            for j in 0..n {
                board[i][j] = match board[i][j] {
                    2 => 0,
                    3 => 1,
                    x => x,
                };
            }
        }
    }
}