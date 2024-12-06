# Rusty LCurve

A TUI for keeping you on track as you practice LeetCode problems using spaced repetition. Recommends problems according (roughly) to the Ebbinghaus Forgetting Curve. Built using ratatui and rusqlite.

## Usage

There are four "modes" you can alternate between:

1. Normal Mode: The default mode. Use left arrow (or `h`) and right arrow (or `l`) to alternate between different tabs. Press `i` to enter Input Mode. Press `u` to enter update mode. Press `e` to enter edit mode. Press `q` to quit.
2. Input Mode: Allows you to enter a new LeetCode problem into the database. Use the left and right arrows to toggle which input box to write to. Use the up and down arrows to select a category of problem. The categories are NeetCode's problem-types. Press `enter` to input the problem. Press `esc` to enter Normal mode.
3. Update Mode: Updates a problem in 'Todays Problems' by incrementing the practice count and recording the current moment as the time you last practiced the problem. Use the up and down arrows to select a problem to update. Press `enter` to update the problem. Press `esc` to enter Normal mode.
4. Edit Mode: For use in the second tab. Gives you a more granular view of the items in your database.
