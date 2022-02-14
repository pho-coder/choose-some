#[cfg(test)]
mod tests {

    #[test]
    fn test_test() {
        let v = vec![10, 20, 30];
        for i in v {
            print!("{}", i);
        }
        assert_eq!(1, 1);
    }
}
