use std::thread::scope;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;
    let (left, right) = v.split_at(midpoint);
    let total = scope(|scope| {
        let handle_left = scope.spawn(|| -> i32 {
            left.iter().sum()
        });
        let handle_right = scope.spawn(|| -> i32 {
            right.iter().sum()
        });
        handle_left.join().unwrap() + handle_right.join().unwrap()
    });
    total
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
