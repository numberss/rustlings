fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    // basic for loop
    // let mut result = 1;
    // for x in 1..=num {
    //     result *= x;
    // }
    // result
    
    // .product() multiplies all the values of an iterator.
    (1..=num).product()
    
    // fold applies an accumulator (the range we set before) onto an initial.
    // here the range starts at 1, going until num.
    // we create "acc" and "x" variables and apply them to each other.
    // acc takes the value of the range we are on, 
    // and x takes the value of fold value we are applying to.

    // e.g start at 1. acc = 1, x = 1. 1 * 1 = 1.
    // acc = 2, x = 1. 2 * 1 = 2.
    // acc = 3, x = 2. 3 * 2 = 6.
    // acc = 4, x = 6. 4 * 6 = 24.
    // etc.
    // (1..=num).fold(1, |acc, x| acc * x)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
