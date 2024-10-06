pub fn fibonacci(n: u32) -> u32 {
    let i = n as usize;
    let mut fibs: Vec<u32> = vec![0, 1];
    while fibs.get(i).is_none() {
        let num = fibs[fibs.len() - 1] + fibs[fibs.len() - 2];
        fibs.push(num);
    }
    *fibs.get(i).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
