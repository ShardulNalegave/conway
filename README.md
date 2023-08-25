
# Game of Life
Game of Life or simply life is a zero-player game devised by British mathematician John Horton Conway in 1970.
The functioning of the game depends only on its initial state and is governed by very simple rules.

Conway's Game of Life is Turing complete, it's halting problem is whether a given initial configuration becomes stable after `n` generations or if it keeps evolving.

![Glider](https://github.com/ShardulNalegave/conway/blob/main/assets/glider.mp4?raw=true)

## Rules
- If a live block has less than 2 live neighbours, then the block dies by underpopulation.
- If a live block has 2/3 live neighbours, it persists.
- If a live block has more than 4 live neighbours, it dies due to overpopulation.
- If a dead block has exactly 3 live neighbours, it becomes live due to reproduction.

## How to run
Just clone the repo and run

```bash
cargo run
```

## Key-Bindings
- `n` -> Next Generation
- `s` -> Show Neighbours (Shows the number of neighbours each block has)
- `q` -> Quit