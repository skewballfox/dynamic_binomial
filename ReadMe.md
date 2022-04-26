## Dynamic Binomial

a comparison of the relative performance of 2 binomial coefficient functions using bottom-up dynamic programing and memoization


## About the benchmark

### The function implementations

both versions of the implementation are based on the functions from  the geeks for geeks page on [calculating the binomial coefficient using dynamic programming](https://www.geeksforgeeks.org/binomial-coefficient-dp-9/). the bottom up is based on the space efficient c++ implementation, and the memoized version is mostly based on the memoized implementation, with a few changes due to the constraints of rust and the data type used, namely all values are `Option<u64>`. At the time, this made sense given that each value was either returned or instantiated, so it seemed reasonable to make that a value which could either be none or a solution to a given subproblem. the bottom-up version is iterative, wherease the memoized function is recursive.

I also included a naive implementation that I was benchmarking against the two DP implementations, and had I started this benchmark two weeks ago, I might have let it run to completion. I had let it run for a day, and about 1/4th of the way through the total number of iterations, the estimated time to collect a given sample set was around 5800 seconds (about 1 hour 26 minutes); at that point I decided brevity was the soul of wit.

### Tools used

I benchmarked these functions using the [criterion](https://github.com/bheisler/criterion.rs) library, using the black_box function to prevent compile time optimizations which may change the nature of the implementation. 

### How to run this benchmark yourself

assuming that you have [installed rust locally](https://www.rust-lang.org/tools/install)

```bash

git clone https://github.com/skewballfox/dynamic_binomial &&\
cd dynamic_binomial &&\
cargo bench

```
then I recommend going for a walk and/or contemplating life

this will yield a lot more performance data than was listed here, including violin plots of the two versions side by side, actual numeric upper and lower bounds per iteration, among many other potentially useful bits of data.

plus walking for a few hours is probably good for you.

## Results

### Functions

#### Bottom-up binomial coefficient

![bottom_up_binomial Coefficient](./assets/bottom_up_lines.svg#gh-light-mode-only)
![bottom_up_binomial Coefficient](./assets/bottom_up_lines_dark.svg#gh-dark-mode-only)


#### Memoized binomial coefficient

![Memoized Binomial Coefficient](./assets/memoized_lines.svg#gh-light-mode-only)
![Memoized Binomial Coefficient](./assets/memoized_lines_dark.svg#gh-dark-mode-only)

## Interpretation

### Unexpected but interesting.

So the bottom-up version was actually faster, and the behavior appears to be more consistent: the easier iterations(where the solution was simple) seemed to gradually increase in difficulty, (they outline y=sqrt(x)?) . 

this was not the result I was expecting, or the result I was told to expect. I believe that this is due to the implementation details: choice of data structure, use of recursion, and the number of subproblems for this algorithm. Making sense of these details is necessary for making an ideal implementation.

### Why was memoization comparitively slow?

#### Devils in the details

first, let's compare the size of a u64 and Option<u64> using the [size_of function from the standard library](https://doc.rust-lang.org/std/mem/fn.size_of.html). the following is just a printout of the size of these two types side by side:

```
size of u64: 8, size of Option<u64>: 16
```


## Conclusion and Ideas for further testing

remove black box testing (recursion to iterative conversion)(understand real world behavior)

pick a smarter set of (n,k) for testing, lower n and convert to take/return i64

remove option and use -1 as indicator that unitialized.

