
int* spiralOrder(int** matrix, int matrixSize, int* matrixColSize, int* returnSize) {
    int rows = matrixSize;
    int cols = matrixColSize[0];

    int *res = (int *)malloc(rows * cols * sizeof(int));
    *returnSize = 0;

    int top = 0, bottom = rows - 1;
    int left = 0, right = cols - 1;

    while (left <= right && top <= bottom) {
        for (int j = left; j <= right; j++)
            res[(*returnSize)++] = matrix[top][j];
        top++;

        for (int i = top; i <= bottom; i++)
            res[(*returnSize)++] = matrix[i][right];
        right--;

        if (top <= bottom) {
            for (int j = right; j >= left; j--)
                res[(*returnSize)++] = matrix[bottom][j];
            bottom--;
        }

        if (left <= right) {
            for (int i = bottom; i >= top; i--)
                res[(*returnSize)++] = matrix[i][left];
            left++;
        }
    }

    return res;
}