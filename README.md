### today-puzzle

I was intrigued by this puzzle at a local math event.

![Today-is math puzzle](today-puzzle.jpg)

So I decided to write a solution for it.

Converted to an 8x8 grid, the board is layed out like this
(where XX is a square outside the board):

```
Ja Fe Ma Ap Ma Ju XX XX
Ju Au Se Oc No De XX XX
01 02 03 04 05 06 07 XX
08 09 10 11 12 13 14 XX
15 16 17 18 19 20 21 XX
22 23 24 25 26 27 28 XX
29 30 31 XX XX XX XX XX
```

This program will solve it for a given day by outputting a solution like this (e.g. for Dec 25):

```
A A G G G G X X
A A E E G X X X
A A E F B B B X
D D E F B H B X
C D E F F H H X
C D D X F H H X
C C C X X X X X
X X X X X X X X
```

In the solution, 8 piece shapes are denoted by letters A-H. X is a space not covered by the piece (i.e. the target date or outside the board).

If you're just looking for solutions:
- [first_solutions.txt](first_solutions.txt) has one solution for each day of the year
- [count_solutions.txt](count_solutions.txt) has the number of solutions for each day of the year

### Notes

- Oct 6 has the fewest unique solutions: 7
- Jan 20 has the most unique solutions: 195
- On my laptop (Dell XPS 13 Plus)
  - it takes about 1 second to find all the solutions for a given date
  - it takes about 10 seconds to find one solution for every day of the year

### Build/Run

today-puzzle is written in [Rust](https://rustup.rs/)

```
# Build in release mode (much much faster than debug mode)
$ cargo build --release

# Print help
$ target/release/today-puzzle --help
Usage: today-puzzle [OPTIONS]

Options:
  -d, --date <DATE>    Date to solve in MM-DD format [default: today]
  -a, --all-dates      Count solutions for every day of the year
  -p, --print <PRINT>  Specifies with solutions to print [default: first] [possible values: first, summary, all, count]
  -h, --help           Print help information


# Run to solve a specific date
$ target/release/today-puzzle --date 12-25
**** 12-25 ****
A A G G G G X X
A A E E G X X X
A A E F B B B X
D D E F B H B X
C D E F F H H X
C D D X F H H X
C C C X X X X X
X X X X X X X X

# Run for all days, but only print the solution count
$ target/release/today-puzzle --all-dates --print count
01-01 has 64 solutions
01-02 has 109 solutions
01-03 has 47 solutions
01-04 has 103 solutions
...snip...
```
