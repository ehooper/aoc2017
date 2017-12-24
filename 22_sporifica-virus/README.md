Visualizing the automaton
=========================

You can see an animation of the solution by enabling the `visualization` feature during build and passing a `--visual` argument to the program for either part one or part two.

Caution: I wasn't so careful with this, so there is a chance you'll want `stty sane` (or your OS equivalent) handy in case it messes up your shell. Also, visualization doesn't support piped input.

Example: `cargo run --features visualization -- --visual part1`

When compiled with the visualization feature, you can pass `-h` to see the help for the command line options.
