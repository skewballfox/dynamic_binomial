///you don't need to be smart when you're strong
pub fn naive_bin_coeff(num_objects: u64, num_selections: u64) -> u64 {
    if num_selections == 0 || num_selections == num_objects {
        return 1;
    } else {
        return naive_bin_coeff(num_objects - 1, num_selections)
            + naive_bin_coeff(num_objects - 1, num_selections - 1);
    }
}

// p.s. the following implementations are heavily based off the memory efficient implementation
// and the memoized version on the related geeksforgeeks page
// https://www.geeksforgeeks.org/binomial-coefficient-dp-9/

///computes a binomial coefficient using bottom up dynamic programming
/// the bottom up strategy is start with the smallest subproblem and work our
/// way to the largest
pub fn bottom_up_bin_coeff(num_objects: u64, num_selections: u64) -> u64 {
    if num_selections == 0 || num_selections == num_objects {
        return 1;
    };

    let mut sub_solutions: Vec<u64> = vec![0; num_selections as usize + 1];
    sub_solutions[0] = 1;
    //these are just placeholders because the normal C[i]=C[i]+C[i+1] is
    //problematic in rust, and I generally avoid variable declarations inside
    //loops
    let mut x: u64 = 0;
    let mut y: u64 = 0;

    for out_i in 1..=num_objects as usize {
        let bound = out_i.min(num_selections as usize);
        (1..=bound).rev().for_each(|in_i| {
            x = sub_solutions[in_i];
            y = sub_solutions[in_i - 1];
            sub_solutions[in_i] = x + y;
        });
    }
    let res = sub_solutions[num_selections as usize].clone();

    res
}

///compute the binomial coefficient using a lookup table to cache solutions to subproblems
pub fn memoized_bin_coeff(num_objects: u64, num_selections: u64) -> u64 {
    let mut lookup_table: Vec<Vec<Option<u64>>> =
        vec![vec![None; num_selections as usize + 1]; num_objects as usize + 1];
    _memoized_bin_coeff(
        num_objects as usize,
        num_selections as usize,
        &mut lookup_table,
    )
}

fn _memoized_bin_coeff(
    num_objects: usize,
    num_selections: usize,
    //this type declaration is saying "borrow this reference to a block of memory, and you have permission to alter it"
    lookup_table: &mut Vec<Vec<Option<u64>>>,
) -> u64 {
    //if the value exist in the table, return it
    if let Some(x) = lookup_table[num_objects][num_selections] {
        return x.clone();
    }

    //base case 1: k is 0
    if num_selections == 0 {
        lookup_table[num_objects][0] = Some(1);
        return 1;
    }

    //base case 2: k is num_selections
    if num_selections == num_objects {
        lookup_table[num_objects][num_selections] = Some(1);
        return 1;
    }

    // otherwise compute
    let x = _memoized_bin_coeff(num_objects - 1, num_selections, lookup_table);
    let y = _memoized_bin_coeff(num_objects - 1, num_selections - 1, lookup_table);
    lookup_table[num_objects][num_selections] = Some(x + y);
    return x + y;
}

#[cfg(test)]
mod test {
    use super::memoized_bin_coeff;

    use super::bottom_up_bin_coeff;

    #[test]
    fn bottom_up_bin_coeff_test() {
        assert_eq!(bottom_up_bin_coeff(20, 10), 184756);
        assert_eq!(bottom_up_bin_coeff(33, 13), 573166440);
    }
    fn memoized_bin_coeff_test() {
        assert_eq!(memoized_bin_coeff(20, 10), 184756);
        assert_eq!(memoized_bin_coeff(33, 13), 573166440);
    }
}
