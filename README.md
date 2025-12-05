# Advent of Code 2025
This year's goal is to put what I've learned in rust over the past few years to practice.  
I'll be creating the solutions manually myself first, then I'll be using [Zed's](https://zed.dev/) agentic capabilities to see if an AI can solve it.  

Here is the initial prompt I'm using for the AI:  
````
Solve the following Advent of Code challenge.  
```  
<Part 1 text>  

```  
Put the solution in [@part1_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part1_ai.rs)   
You can test the program by running `cat data/input.txt | cargo run --bin part1_ai`
````  

And for part 2 (in the same thread):  
````
That is the correct answer.  
Now solve part 2:  

```  
<Part 2 text>  

```  
Put the solution in [@part2_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part2_ai.rs)   
You can test the program by running `cat data/input.txt | cargo run --bin part2_ai`
````

### Day 1  
Using Claude Sonnet 4.5 via [OpenRouter](https://openrouter.ai/), both parts were solved in one go with the provided prompts.  
Total cost: €1.08  

### Day 2  
I had to extend the prompt to tell the agent not to explore the repo, as it found my code and decided to copy paste the solution instead of doing it itself.  
```
Do not explore the codebase before attempting to create your solution.  
```

After that, it was able to solve both parts with one try.  
Cost: €0.76  

### Day 3
For part 1, it created a working solution the first time. However it used a brute force approach, trying every combination of possible batteries and finding the best.  
For part 2, it also created a working solution first time.  

Cost: €0.48  

### Day 4  
For both parts, it created a working solution first time.  
For part 2, it decided to run part1 -> remove accessible rolls -> repeat until no more accessible rolls.  

Cost: €0.47  

For my solution, I used:  
- A [Summed Area Table](https://en.wikipedia.org/wiki/Summed-area_table) to compute neighbourhoods. I've used this to implement fast box blurring in the past. It's overkill for this, but would scale better with neighbourhood size.  
- A breadth-first search for removing rolls. This results in less wasted computation as it only re-checks areas around removed rolls. This would win out when the grid is sparse or there are big chains of knock-on effects when removing rolls.  

### Day 5  
I found the AI was struggling to pipe the input from the example into the program, so I updated the prompt with:  

```
You can test the program by running:  
- `cat data/example.txt | cargo run --bin part1_ai` (example input shown above)  
- `cat data/input.txt | cargo run --bin part1_ai` (real input)  
```

For part 1, it almost created a working solution, but ran into an overflow issue (as did I) using u32 instead of u64. It quickly resolved it.  
Part 2 it was able to get in one shot, taking the same approach as I did - Merging overlapping ranges before counting.  
Cost: €0.54  


---
Here is the latest prompt I'm using for the AI:  
````
Solve the following Advent of Code challenge.  
```  
<Part 1 text>  
```  
Do not explore the codebase before attempting to create your solution.  
Put the solution in [@part1_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part1_ai.rs)   
You can test the program by running:  
- `cat data/example.txt | cargo run --bin part1_ai` (example input shown above)  
- `cat data/input.txt | cargo run --bin part1_ai` (real input)  
````  

And for part 2 (in the same thread):  
````
That is the correct answer.  
Now solve part 2:  

```  
<Part 2 text>  
```  
Do not explore the codebase before attempting to create your solution.  
Put the solution in [@part2_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part2_ai.rs)   
You can test the program by running:  
- `cat data/example.txt | cargo run --bin part2_ai` (example input shown above)  
- `cat data/input.txt | cargo run --bin part2_ai` (real input)  
````

