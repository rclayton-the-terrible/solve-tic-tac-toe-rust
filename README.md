# Solve Tic-Tac-Toe

This is a really simple example project (in Rust -- because I'm trying to learn the language) on ways to evaluate the results of a Tic-Tac-Toe game.

This is a common question asked during technical interviews ([I saw this on Reddit](https://www.reddit.com/r/programming/comments/miuam0/a_google_interview_question_determine_if_someone/)) and it made me think, "how would I go about doing it?"

In this repo, I tackle the problem using both loops and bitwise operators.  The [original blogger](https://jrms-random-blog.blogspot.com/2021/03/a-google-interview-question.html) mentioned a third option (hashing all possible winning combos and looking up the result), I'm just not sure how I would attempt that in a way that is novel from the other approaches (I probably misunderstand the process).

Given that there are only 8 winning combinations in Tic-Tac-Toe, we can evaluate whether players' board state matches a winning combination.

This can be done using arrays using nested loops:

```
for each winning combo [C]:
  for each value in the combo [c]:
    if current value at position is X:
      increment X matches
    if current value at position is O:
      increment O matches
  if X matches == 3
    return X as winner
  if O matches == 3
    return O as winner
default: return no winner
```

The alternate way is to use bitwise checks:

```
assuming you have a constant array of 8 winning positions as bit arrays (really, just ints)
convert the board positions for X's and O's into bit arrays (ints)
for each winning combo (as int):
   if the bitwise & of X and combo == combo (meaning all in combo match in X):
     return X as winner
   if the bitwise & of O and combo == combo (meaning all in combo match in O):
     return O as winner
default: return no winner
```

So what is the performance difference?

```
> cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests (target/release/deps/solve_tic_tac_toe-2ba3cdb4b32fcc5e)

running 5 tests
test bit_strategy::tests::test_board_to_bits ... ignored
test tests::tests::test_bit_wise_eval_winner ... ignored
test tests::tests::test_loop_eval_winner ... ignored
test tests::tests::bench_bit_wise_eval_winner ... bench:          97 ns/iter (+/- 4)
test tests::tests::bench_loop_eval_winner     ... bench:         581 ns/iter (+/- 44)

test result: ok. 0 passed; 0 failed; 3 ignored; 2 measured; 0 filtered out; finished in 10.38s
```

The bitwise evaluator is much faster and has less variability in performance!
