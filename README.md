# Solver for Zoysii
Solve a 4x4 Zoysii level with the lowest number of moves.

Perform a breadth-first search to find the shortest path of actions where `board.is_won()`.
For move $m$ this brute-force search considers $O(4^m)$ board states. However, the search is faster than this upper bound by pruning invalid moves, duplicates and `board.is_lost()`.


## Run
```bash
> cargo run -r "18 9 6 0|0 9 3 0|33 18 18 3|0 0 15 0"
Solution with 13 moves: Right, Down, Right, Down, Down, Up, Left, Left, Up, Down, Right, Right, Up


> cargo run -r "18 9 6 36|0 9 3 0|33 18 18 3|36 18 15 9"
Solution with 17 moves: Down, Up, Right, Right, Down, Up, Left, Down, Down, Down, Left, Up, Right, Right, Down, Right, Up
```
Note that allowed cell values range from 0 to 255.
Moreover, there is a limit of 29 moves.

# References
- [Zoysii](https://gitlab.com/deepdaikon/Zoysii) by deepdaikon
