## Dynamic Binomial

a comparison of the relative performance of 2 binomial coefficient functions using bottom-up dynamic programing and memoization

P.S. Thanks to input from  DuckThatSits and 8-bit Zeta from discord, I'll be updating this writeup soon with more context and potential improvements to the memoized version. 

I'll also eventually be running the benchmarks again after making those improvements, but finals are coming up and can't currently afford the CPU time.

## About the benchmark

### The function implementations

both versions of the implementation are based on the functions from  the geeks for geeks page on [calculating the binomial coefficient using dynamic programming](https://www.geeksforgeeks.org/binomial-coefficient-dp-9/). the bottom up is based on the space efficient c++ implementation, and the memoized version is mostly based on the memoized implementation, with a few changes due to the constraints of rust and the data type used, namely all values are `Option<u64>`. At the time, this made sense given that each value was either returned or instantiated, so it seemed reasonable to use a type where the value could either be `None` or a solution to a given subproblem. the bottom-up version is iterative, wherease the memoized function is recursive.

I also included a naive implementation that I was benchmarking against the two DP implementations, and had I started this benchmark two(maybe three) weeks ago, I might have let it run to completion. I had let it run for a day and a half, and about 1/4th of the way through the total number of iterations, the estimated time to collect a given sample (around 100 iterations after warm up) was around 5800 seconds (about 1 hour 26 minutes); at that point I decided brevity was the soul of wit.

### Tools used

I benchmarked these functions using the [criterion](https://github.com/bheisler/criterion.rs) library, using the black_box function to prevent compile time optimizations which may change the nature of the implementation. All code was compiled and ran on fedora 35, which *may* have impacted the results

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

p.s. apparently my graphical session crashed last night, ending my user session; this benchmark may or may not have been related. 

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

So the bottom-up version was actually faster, and the behavior appears to be more consistent: the easier iterations(where the solution was simple) seemed to gradually increase in difficulty, but with very few outliers. 

this was not the result I was expecting, or the result I was told to expect. I believe that this is due to the implementation details: choice of data structure, use of recursion, and the number of subproblems for this algorithm. Making sense of these details is necessary for making an ideal implementation.

### Why was memoization comparitively slow?

#### Devils in the details

first, let's compare the size of a u64 and Option<u64> using the [size_of function from the standard library](https://doc.rust-lang.org/std/mem/fn.size_of.html). the following is just a printout of the size of these two types side by side:

```
size of u64: 8, size of Option<u64>: 16
```
and let `n` be the number of objects, `k` the number of selections, 
then 
the bottom up version was allocating `8*k` per function call,

the memoized version was allocating `16*n*k` per function call. 

because the size of the allocation depended on both n and k, and the size of the underlying data type was double the size of the bottom up version, it seems likely that part of the instability we observe may be due to [page faults](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux_for_real_time/8/html/reference_guide/chap-memory_allocation#:~:text=A%20potential%20source%20of%20memory%20latency%20is%20called%20a%20minor%20page%20fault).

second, lets review the last few lines of the memorized DP version:
```rs
   let x = _memoized_bin_coeff(num_objects - 1, num_selections, lookup_table);
    let y = _memoized_bin_coeff(num_objects - 1, num_selections - 1, lookup_table);
    lookup_table[num_objects][num_selections] = Some(x + y);
    return x + y;
```

I set set the output of the two function calls to separate variables, as each function call needs to *borrow* the lookup table. if you use c++ often, the concept of ownership is similar to a [unique_pointer](https://docs.microsoft.com/en-us/cpp/cpp/how-to-create-and-use-unique-ptr-instances?view=msvc-170), save for the fact that a value can't be null in rust.  `lookup_table` can only have one owner at a time. I was trying to avoid having a statement where lookup table was being borrowed by multiple function calls, lest I invoke the wrath of the borrow checker. This may or may not have been necessary(Murphey would be proud), but either way this implementation detail means that some computation was happening after the recursive function calls and --due to the black_box benchmark-- the compiler made no attempts to optimize this away.

third, there may be some overhead with the use of `Option<u64>` compared to `u64`, I don't believe this should make a substantial difference, but honestly I'm not sure, and currently having trouble finding documentation that could help confirm one way or another.

## Conclusion and Ideas for further testing

First, I'd like to refactor the memoized version to see if there is a way to avoid doing any work after the function call other than returning the value. I'd also like to convert the function parameters and return type to i64. This would me a reduction of the possible values of (n,k) that could work(max n would now be 66), but that would allow -1 to be used as an indicator, and reduce the size of the lookup table by half. it would also remove any performance overhead due to using option instead of the types `i64`/`u64` directly.

I'd like to also run a set of benchmarks that aren't preventing the compiler from performing any optimizations, as it would be interesting (and perhaps even useful!) to see how the behavior of these two implementations changes when put in a form closer to what would be observed in the real world.

also, to avoid long meditative moments of self reflection, I want to come up with a smarter selection of values of (n,k), that still cover the same range of outputs but isn't, in fact, every possible pair of (n,k) which can safely be assumed not to result in a runtime panic due to overflow and isn't trivial to solve(k={1|n})
