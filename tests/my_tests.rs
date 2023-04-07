use rust_sample::func_a;
use rust_sample::func_b;

// lib.rs はライブラリのエントリポイントとなるらしい

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_a() {
        assert_eq!(1, func_a());
    }

    #[test]
    fn test_func_b() {
        assert_eq!('a', func_b());
    }
}
