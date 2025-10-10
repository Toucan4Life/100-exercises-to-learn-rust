// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let leaked_vector: &'static mut [i32] = v.leak();
    let (start_half, end_half) = leaked_vector.split_at(leaked_vector.len() / 2);

    let handle_start = thread::spawn(move || {
        start_half.iter().sum::<i32>()
    });
    let handle_end = thread::spawn(move || {
        end_half.iter().sum::<i32>()
    });

    let first_half_result: i32 = handle_start.join().unwrap();
    let second_half_result: i32 = handle_end.join().unwrap();

    first_half_result + second_half_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
