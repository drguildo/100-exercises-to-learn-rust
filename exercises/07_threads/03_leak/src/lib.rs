use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let slice: &'static mut [i32] = v.leak();
    let (left, right) = slice.split_at(slice.len() / 2);
    let handle_left = thread::spawn(|| {
        left.iter().sum::<i32>()
    });
    let handle_right = thread::spawn(|| {
        right.iter().sum::<i32>()
    });
    handle_left.join().unwrap() + handle_right.join().unwrap()
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
