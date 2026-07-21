from typing import List

class Solution:
    def gameOfLife(self, board: List[List[int]]) -> None:
        m, n = len(board), len(board[0])

        for i in range(m):
            for j in range(n):
                live = 0
                for x in range(max(0, i - 1), min(m, i + 2)):
                    for y in range(max(0, j - 1), min(n, j + 2)):
                        if (x != i or y != j) and board[x][y] in (1, 2):
                            live += 1
                if board[i][j] == 1:
                    if live < 2 or live > 3:
                        board[i][j] = 2
                elif live == 3:
                    board[i][j] = 3

        for i in range(m):
            for j in range(n):
                board[i][j] %= 2