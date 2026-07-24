impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut top = 0;
        let mut bottom = matrix.len() as i32 - 1;
        let mut left = 0;
        let mut right = matrix[0].len() as i32 - 1;

        while left <= right && top <= bottom {
            for j in left..=right {
                res.push(matrix[top as usize][j as usize]);
            }
            top += 1;

            for i in top..=bottom {
                res.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for j in (left..=right).rev() {
                    res.push(matrix[bottom as usize][j as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for i in (top..=bottom).rev() {
                    res.push(matrix[i as usize][left as usize]);
                }
                left += 1;
            }
        }

        res
    }
}