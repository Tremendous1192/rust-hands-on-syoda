// ▼ List 4-32
// Public function 'add' that takes two 'usize' parameters and returns their sum
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Module for unit tests
#[cfg(test)]
mod tests {
    // Import symbols from the outer scope, including the 'add' function
    use super::*;

    // Test case to verify the 'add' function
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // ▼ List 4-40
    // Test case to check if numbers in the 'data' array are prime using 'calc::is_prime'
    #[test]
    fn it_is_prime() {
        let data = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for n in data {
            let res = calc::is_prime(n);
            assert_eq!(res, true);
        }
    }

    // Test case to check if numbers in the 'data' array are not prime using 'calc::is_prime'
    #[test]
    fn it_isnot_prime() {
        let data = [4, 6, 9, 10, 12, 14, 15, 16, 18, 20];
        for n in data {
            let res = calc::is_prime(n);
            assert_ne!(res, true);
        }
    }
}

// ▼ List 4-36
// Public module 'calc' with a public 'add' function and 'is_prime' function
pub mod calc {

    // Function to add two 'usize' parameters and return their sum
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    // ▼ List 4-41
    // Function to check if a number is prime
    pub fn is_prime(num: usize) -> bool {
        let mut f = true;
        for n in 2..=(num / 2) {
            //☆
            if num % n == 0 {
                f = false;
            }
        }
        return f;
    }
}
