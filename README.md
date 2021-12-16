# Advent of Code 2021
This is my first time with the Advent of Code challenge.  This is my code for for the puzzles written in Rust.

I'm a beginner and using this challenge to learn the language! 

### Overall Features
- launches from the command line and takes arguments for the day and an input file.  Calculates the two results.

### Day 1
#### Approach and Unique features
- Proof of concept really.  The challenge was relatively easy and input was one value per line..

### Day 2
#### Approach and Unique features
- Built more data structures.

### Day 3
#### Approach and Unique features

### Day 4: Bingo Game
#### Approach and Unique features
- First challenge with more complex data input with different data separators. 

### Day 5: Map of Sea Vents
#### Approach and Unique features
- For data input, used the `filter_map()` method to deal with the different characters and convert to Rust vectors (similar to arrays in other languages).
- Used the `grid` crate to create a 2D addressable vector structure to map the sea vents. I intended to not use much beyond the standard library.  This was a compromise to just get sane 2D addressing without writing it myself.
- Challenge1: Used loops to put each integer on the grid. othing special. 
- Challenge2: Same as Challenge 1 but with the addition of test logic to subtract index numbers for addressing since Rust can't iterate backwards without re-typing and additional methods.

### Day 6: Lanternfish Growth
#### Approach and Unique features
- Challenge 1: Used a growing vector in the same format as the input data.  This was suitable for the small number of growth cycles in the puzzle.
- Challenge 1: assigned values and spawned new fish with a match operation.
- Challenge 2: Used the standard library `collections::DeqVec` for to be able to roll the vector end to end.

### Day 7: Crab Submarine Fuel Burn
#### Approach and Unique features
- This challenge was more of a problem solving and math exercise rather than an intense programming challenge.
- Challenge 1: Solved by find the median.
- Challenge 2: Found the basic formula that describes the fuel rate.  Loop from the lowest to highest possible values to find the one that has the minimum total fuel use.
- Used a closure in Challenge 2 to hold the fuel burn formula.

### Day 8: Decoding 7 Segment Displays
#### Approach and Unique Features
- The first challenge is a teaser where you just have to count how often specific patterns appear.
- Challenge 1: Text processing and splitting followed by matches on character counts.
