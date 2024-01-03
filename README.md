# ADVENT OF CODE 2023

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges.

This is my first year participating in the Advent of Code, and I'm excited to see how far I can get!

I'm not doing this for getting the best time or anything (Python is probably the best language for that), but rather to have fun.

## Solutions

Those are the solutions I've used to get the answers for the challenges.

| Day | Part 1 | Part 2 |
| --- | ------ | ------ |
| 1 | [Solution](day-1/src/part_1.rs) | [Solution](day-1/src/part_2.rs) |
| 2 | [Solution](day-2/src/part_1.rs) | [Solution](day-2/src/part_2.rs) |
| 3 | [Solution](day-3/src/part_1.rs) | [Solution](day-3/src/part_2.rs) |
| 4 | [Solution](day-4/src/bin/part_1.rs) | [Solution](day-4/src/bin/part_2.rs) |
| 5 | [Solution](day-5/src/part_1.rs) | [Solution](day-5/src/old/part_2.rs) |
| 6 | [Solution](day-6/src/part_1.rs) | [Solution](day-6/src/part_2.rs) |
| 7 | [Solution](day-7/src/part_1/mod.rs) | [Solution](day-7/src/part_2/mod.rs) |
| 8 | [Solution](day-8/src/part_1/mod.rs) | [Solution](day-8/src/part_2/mod.rs) |
| 9 | [Solution](day-9/src/part_1/mod.rs) | [Solution](day-9/src/part_2/mod.rs) |
| 10 | [Solution](day-10/src/part_1/mod.rs) | [Solution](day-10/src/part_2/mod.rs) |
| 11 | [Solution](day-11/src/part_1/mod.rs) | [Solution](day-11/src/part_2/mod.rs) |
| 12 | [Solution](day-12/src/part_1/mod.rs) | [Solution](day-12/src/part_2/mod.rs) |
| 13 | [Solution](day-13/src/part_1/mod.rs) | [Solution](day-13/src/part_2/mod.rs) |
| 14 | [Solution](day-14/src/part_1/mod.rs) | [Solution](day-14/src/part_2/mod.rs) |
| 15 | [Solution](day-15/src/part_1/mod.rs) | [Solution](day-15/src/part_2/mod.rs) |
| 16 | [Solution](day-16/src/part_1.rs) | [Solution](day-16/src/part_2.rs) |
| 17 | [Solution](day-17/src/part_1/mod.rs) | [Solution](day-17/src/part_2/mod.rs) |
| 18 | [Solution](day-18/src/part_1/mod.rs) | [Solution](day-18/src/part_2/mod.rs) |
| 19 | [Solution](day-19/src/part_1.rs) | [Solution](day-19/src/part_2.rs) |
| 20 | [Solution](day-20/src/part_1.rs) | [Solution](day-20/src/part_2.rs) |
| 21 | [Solution](day-21/src/part_1.rs) | [Solution](day-21/src/part_2.rs) |
| 22 | [Solution](day-22/src/part_1.rs) | [Solution](day-22/src/part_2.rs) |
| 23 | [Solution](day-23/src/part_1.rs) | [Solution](day-23/src/part_2.rs) |
| 24 | [Solution](day-24/src/part_1/mod.rs) | unsolved |
| 25 | [Solution](day-25/src/part_1/mod.rs) | Just a mocking of the fact that i din't complete day 24 |

## Redos

I've decided to redo some of the challenges that I didn't like the first time around.

| Day | Part 1 | Part 2 |  Comments |
| --- | ------ | ------ | --------- |
| 4 | [Solution](day-4/src/part_1.rs) | [Solution](day-4/src/part_2.rs) | Recursion was a bad idea. |
| 5 | - | [Solution](day-5/src/part_2.rs) | Brute-forcing my way through the solution took too long. |

## Observations

| Day | Part 1 | Part 2 |
| --- | -------- | ------ |
| 21 |          |I could not figure out on my own. I had to look the "data science" part of the solution up.|
| 24 |          |The amount of linear algebra involved in this one was too much for me. maybe I'll come back to it later.|

## TODO
I'm making a yew app to resolve the challenges.

Any multi-threaded solution will not work since wasm doesn't support it yet.

So i need to refactor:
- [ ] Day 5-part 2
- [ ] Day 16-part 2

I need to get rid of the unwraps so that the app doesn't crash on wrong inputs.

Day 24 part 1 will not work because of the i64 type not being supported by wasm.

Day 18 part 2 might not work in some inputs because of the same reason.