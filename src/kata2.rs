pub fn chop(x: i32, slice: &[i32]) -> i32 {
    match slice.binary_search(&x) {
        Ok(ix) => ix as i32,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod test {
    use super::chop;

    #[test]
    fn test_chop() {
        assert!(-1 == chop(3, &[]));
        assert!(-1 == chop(3, &[1]));
        assert!(0 == chop(1, &[1]));

        assert!(0 == chop(1, &[1, 3, 5]));
        assert!(1 == chop(3, &[1, 3, 5]));
        assert!(2 == chop(5, &[1, 3, 5]));
        assert!(-1 == chop(0, &[1, 3, 5]));
        assert!(-1 == chop(2, &[1, 3, 5]));
        assert!(-1 == chop(4, &[1, 3, 5]));
        assert!(-1 == chop(6, &[1, 3, 5]));

        assert!(0 == chop(1, &[1, 3, 5, 7]));
        assert!(1 == chop(3, &[1, 3, 5, 7]));
        assert!(2 == chop(5, &[1, 3, 5, 7]));
        assert!(3 == chop(7, &[1, 3, 5, 7]));
        assert!(-1 == chop(0, &[1, 3, 5, 7]));
        assert!(-1 == chop(2, &[1, 3, 5, 7]));
        assert!(-1 == chop(4, &[1, 3, 5, 7]));
        assert!(-1 == chop(6, &[1, 3, 5, 7]));
        assert!(-1 == chop(8, &[1, 3, 5, 7]));
    }
}
