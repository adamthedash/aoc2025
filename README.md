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

### Day 6  
Despite my previous "Don't explore the codebase" prompt, the agent decided to overwrite the input data file with its own data, removing trailing whitespace on each line. The solution it created for both parts worked either way.  

A new piece of rust syntax it used: `.map(|line| format!("{:<width$}", line, width = width))` - Good for deciding pad width at runtime.  
Our approaches taken to find problem ranges differed:  
- I found the positions of each operator, which are always left-justified.  
- The AI checked for colummns containing only " " as separators.  

Cost: €0.77  

### Day 7  
This was the first time I've seen the AI spin its wheels a bit before creating a correct solution.  

#### Part 1
It started off with a breadth-first search algorithm, beginning at the beam starting point, then propogating downward until a split point.  
Its first solution missed the case where a beam splits onto the path of a beam above it, leading to both beams being propogated down to the next splitter.  
The AI incorrectly identified that it was erroneously counting the starting beam position and tried to fix it, which didn't work.  
It next identified that the issue comes from when two beams split onto the same position, which is closer but still isn't right.  
It then caught itself before running the example, and decided to only count unique hits on splitters. This did result in a correct answer, but it didn't actually identify what was causing the issue in the first place.  

As an aside (not included in total cost), I asked it to explain why the first solution was incorrect and give me a minimal solution demonstrating it. It gave me:  
```
In my first attempt, I was counting a split **every time a beam hit a splitter**, even if multiple beams hit the same splitter. The correct answer requires counting **each unique splitter that gets hit**, not the total number of beam-splitter collisions.

.S.S.
.....
.^.^.
.....
..^..
.....
```
Its first attempt actually solved this minimal example correctly. In its BFS implementation, it keeps track of visited positions as the starting point of each beam (including the starting point after a split), so that it doesn't propogate two new beams from the same starting point.  
I asked it to highlight why specifically this visited tracking strategy doesn't work. It started confidently working though an example, only to realise that it did work.  
It then changed its mind, and correctly stated the actual issue, but gave a wrong example:  
```
The real issue must be that in the actual long input, we can have beams starting from the **same column** but **different rows**. Since `visited` tracks (row, col) pairs, two beams can pass through the same splitter if they start from different rows in the same column!  

..S..
.....
..^..   <- (2,2) splits=1, creates (2,1) and (2,3)
.....
.^.^.   <- (4,1) splits=2, creates (4,0) and (4,2)
.....   <- (4,3) splits=3, creates (4,2) [visited!] and (4,4)
.........   <- beam from (4,2) travels down
.....^...   <- (8,5) from some other path...
.........
..^..   <- (10,2) gets hit by the beam from (4,2)
..^..   <- (11,2) ALSO gets hit by the same beam!
```
Here it thinks that a beam can pass directly through a splitter and hit one below it. It immediately caught itself, realised it's probably going down a wrong path, then asked to see the input data.  

It continued in circles for some time, correctly stating the issue but creating an example which doesn't demonstrate it.  
Eventually I decided to manually create the example and ask it to walk me through it.  
```
..S..
.....
..^..
.....
.^...
.....
..^..
.....
...^.
.....
```
Which it was able to do:  
```
- Splitter at (2, 2): counted once ✓
- Splitter at (4, 1): counted once ✓
- Splitter at (8, 3): counted **TWICE** ❌ (once from beam at (2,3), once from beam at (6,3))
- Splitter at (6, 2): counted once ✓
```

Cost for this investigation: €0.88  

#### Part 2
For part 2, it went for a recursive solution. Starting at the beginning, it followed the beam down until it hit a splitter. Then it branched off into two paths, running the same thing starting from both split points.  
It produced the correct solution for the example data, however with the real data, the runtime would be infeasible, so I stopped it after 60s.  
It then correctly identified that the issue was excessive recursion, and implemented a memoisation strategy so that sub-branches were only calculated once. This produced the correct answer in short time.  

My approach to this problem was quite different. The AI held the 2d grid in memory, indexing into it during a search. I iterated over the rows, updating a running state of the beams over time.  
This would scale better memory-wise with more rows and should have better cache locality, but would lose out computationally to a search based approach where the grid/beams are sparser.  
For part 2, instead of a recurive solution, I propogated down a count of "how many ways are there to reach this location".  
Even for a relatively short number of rows, the recursive solution would likely run into issues with stack depth. Tail-call optimisation can't help here as there are two recursive paths at each splitting point.  

Total cost: €1.07  

### Day 8  
For this problem, I went way overkill. I implemented a [KDTree](https://en.wikipedia.org/wiki/K-d_tree) which allows fast nearest neighbour search. I spent long time extending this to a sorted nearest neighbour search, which allowed me to iterate over neighbours in increasing distance from a target point. I found [this cool trick](https://stackoverflow.com/a/49456073) for lazily-initalised iterators, which allowed me to construct an efficient lazy tree walk while maintaining a nice `Iterator` interface.  
There's a lot of closure nonsense in my implemenetation, so I'm not sure how much overhead the compiler is able to get rid of.  


For part 1, the AI went for a [union-find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) approach. I hadn't come across this before, but it's for handling a collection of non-overlapping disjoint sets (circuits). In my implementation I was merging sets manually.  
Notably it also just calculated the pairwise distances exhaustively, which even for the real input data (N=1000) was very fast.  
It struggled a bit with using 10 connections for the example and 1000 for the real data, but after that it came to the right answer.  

Total cost: €1.12  

### Day 9  
For this problem, I spent some time thinking about different solutions, but in the end I couldn't come with much better than exhaustively checking every possible combination. This problem probably falls into some well-known category which I'm not aware of.  

The AI went straight for a brute force approach for part 1, which it got correct. For part 2, it tried to enumerate every possible green/red tile location, which was prohibitively expensive so I had to kill it after 60s.  
After taking a look at the input data, it recognised that the coordinates were too large to materialise all possible locations up front, so it switched to checking for valid files on the fly. Before running on the real data, it guessed that it would likely still run for way too long, and proposed some potential optimisations. At this point it also started compiling in release mode.  
It then re-introduced the problem by caching checks for "valid" tiles (green/red), meaning we're back to materialising all tiles.  
It then tried to get around the problem by discarding rectangles which were too large. This obviously resulted in an incorrect answer. It upped its limit for rectangle size several times.  
At this point I suggested that it calculate the actual biggest rectangle, rather than brute forcing with different limits. It still kept going in circles, so I had to explicitly tell it to scrap its current approach.  
At this point I noticed that the cost was starting to ramp up quickly. I was up over €3 just for this question and running low on credits on OpenRouter.  
At this point it seemed that either Zed's Agent or Claude Sonnet 4.5 was having issues with context. In its thinking text, there were sections like this:  
```
 I cannot complete the thought because the next thinking appears to be incomplete or truncated. There are no substantive details to help me finish or rewrite the current partial thinking. I apologize, but the provided context appears to be incomplete or fragmented. 
```
I decided that I'd stop the thread here, creating a new one with Zed's "New Thread from Summary" feature.  
Here is the prompt I gave it at this point:  
````
Solve the following Advent of Code challenge. 
```
<Part 1 text>  
<Part 2 text>
```
You have already solved part 1. Your solution is available in [@part1_ai.rs](parth/to/part1_ai.rs)  
Solve part 2 of the challenge.  

Do not explore the codebase (except viewing your part 1 solution) before attempting to create your solution.  
Put the solution in [@part1_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part1_ai.rs)   
The input data has already been prepared.  
You can test the program by running:  
- `cat data/example.txt | cargo run --bin part2_ai` (example input shown above)  
- `cat data/input.txt | cargo run --bin part2_ai` (real input)  
````

For its 2nd attempt, it came up with a better approach. Instead of checking if all points inside each box were on a red/green tile, it checked if the rectangle geometry was inside the polygon geometry.  
However instead of using an exact solution, it decided to sample points along the bounding box perimeter, and check that all of them were inside the polygon.  
It first used a relatively low sample size, resulting in a much too big answer. With an increasd size it got the correct answer. However because it changed, it second guessed itself and decided to use a "more precise" geometric check. As it turns out, this check was simply to add more sample points inside the bounding box.  

Total cost: €4.49  


### Day 10
Part 1 of today was easy enough. Since buttons toggle the lights, each needs to be pressed 0 or 1 times. I chose to store the button patterns as integers and use some bitwise operations to enumerate all possible combinations.  
Part 2 I first tried to use some heuristics to solve for the correct distribution of presses, but I kept running into cases where it failed. Eventually I took a step back and went down the [Constraint optimisation](https://en.wikipedia.org/wiki/Constraint_programming) route.  
Given a rough initial set of constraints, I refined them using [Gaussian elimination](https://en.wikipedia.org/wiki/Gaussian_elimination) which gave me a simpler set which could be enumerated using a [Backtracking](https://en.wikipedia.org/wiki/Backtracking) algorithm. I had to fiddle around with the ordering in which constraints were checked and variables enumerated to reduce the search space down to a feasible size.  

The AI went for a similar solution to me for part 1. It did note this problem was equivalent to a "linear algebra problem over GF(2)".  
For part 2, the AI initially tried using Dijkstra's to find the minimum cost path to the target state. This timed out, presumably because the search space is too large due to roughly evenly-weighted buttons.  
As its 2nd attempt, it tried to go for a linear programming approach: "finding the minimum L1 norm solution to a system of linear equations". Here it ran into integer overflow issues and failed to fix them for a few tried. It managed to get a running program but this also timed out.  
For its 3rd attempt, it went for "A mathematical solution using gaussian elimination or a greedy heuristic". It definitely didn't implement GE, but from what I can tell it's a breadth-first search based solution. Again this timed out.  
At this point it took a look at the data, and decided on a "greedy approach with backtracking". The approach was similar again. And again it timed out. It went around a bit more in circles trying slightly different variations of the same thing, but each time with "more optimial" solutions.  
Similar to yesterday, I decided to start a new thread with a summary and see if it could get itself out of the ditch. I used the same prompt as above.  

Immediately, it went back to trying the same thing again, so I'm not entirely sure Zed's summary feature is working.  
One weird thing is that it mentioned using gaussian elimination multiple times, but never actually implements it and instead goes back to using greedy approaches. I decided to wipe the chat history and be explicit about using GE.  

```
Treat this problem as solving a system of linear equations. Use gaussian elimination to first simplify the constraints. Then use whatever method you would like to enumerate the solution space to find the best solution. Several cases in the input space have large search spaces, so brute force algorithms will not work. Use one which will help you narrow down the search space without fully exploring it.  
```

I also added an explicit timeout to the testing commands so I wouldn't need to manually kill it.  

```
- `cat data/example.txt | timeout 60 cargo run --bin part1_ai` (example input shown above)  
- `cat data/input.txt | timeout 60 cargo run --bin part1_ai` (real input)  
```


This time, it actually tried to use a GE approach. However, on getting the wrong answer, it immediately scrapped it and went back to a search based solution. Even with me trying to steer it on track to fix its GE implementation, it kept going in circles.  
At this point I had to call it quits, the agent failed to solve part 2.  

Total cost: €9.54  

### Day 11
Today was a fun puzzle, a graph traversal problem. Instead of going with a search algorithm, I decided to use [this cool property](https://en.wikipedia.org/wiki/Adjacency_matrix#Matrix_powers) of the adjacency matrix. Raising the matrix to the power N gives you the paths of length N between all points on the graph.  
Initially I created a dense matrix, but due to the O(N^3) complexity of a matrix multiplication, I had to use sparse data instead. This also had the added bonus of reduced complexity when computing paths between individual points, in this case only 6 of the 500^2-ish pairs of possible points.  

The AI went for a simple DFS algorithm for part 1. 
For part 2, it initially tried its part 1 solution, but with checks to see which paths pass through the correct nodes. This timed out.  
It then tried to add some memoisation for visited paths, but this didn't help much as the graph is quite large. This also timed out.  
It stated several times that maybe it needs to use dynamic programming, then went back and tried DFS with memoisation again.  
Eventually it did succeed with this approach after assuming that the graph is a DAG, and was able to get rid of cycle tracking which was evidently taking most of the time.  

Total cost: €2.13  



---
Here is the latest prompt I'm using for the AI:  
````
Solve the following Advent of Code challenge.  
```  
<Part 1 text>  
```  
Do not explore the codebase before attempting to create your solution.  
Put the solution in [@part1_ai.rs](file:///home/adam/projects/rust/aoc2025/day1/src/bin/part1_ai.rs)   
The input data files have already been prepared.  
You can test the program by running:  
- `cat data/example.txt | timeout 60 cargo run --bin part1_ai` (example input shown above)  
- `cat data/input.txt | timeout 60 cargo run --bin part1_ai` (real input)  
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
The input data files have already been prepared.  
You can test the program by running:  
- `cat data/example.txt | timeout 60 cargo run --bin part2_ai` (example input shown above)  
- `cat data/input.txt | timeout 60 cargo run --bin part2_ai` (real input)  
````

# Some takeaways
- When faced with an error/timeout, the AI often suggested a solution, but then implement something else entirely, often the "easy" path instead of trying to properly solve the problem.  
- The AI almost never looked at the input data, or added debug prints in order to get more information about why an attempt was not working.  
- Within one "thinking cycle" (between attempting to run the program), the AI often caught itself multiple times in short succession. "Wait, I see the issue now", "Actually, I can do XYZ", "Wait, let me reconsider", etc. I'm not sure if this is an artefact of the model I'm using, how context is fed to the model, or something else. When it went down a wrong path (like day 10), it seemed to get stuck in this cycle of second guessing itself, and settling on not changing a whole lot rather than try more different approaches which it identified earlier in the context.  
- Code generated was rarely ever commented. It tended to dump everything into one big function unless some recursion was needed. It didn't really use a whole lot of Rust's niceties, opting for an imperitive style for most things.  


