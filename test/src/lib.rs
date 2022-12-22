#[cfg(test)]
mod tests {
    use try_polyfill::{try_, Try};

    #[test]
    fn try_macro_option() {
        assert_eq!(demo(Some(1)), Some(2));
        assert_eq!(demo(None), None);
    }

    #[test]
    fn try_macro_result() {
        assert_eq!(demo(Ok::<_, ()>(1)), Ok(2));
        assert_eq!(demo(Err("err")), Err("err"));
    }

    fn demo<T: Try<Continue = i32>>(t: T) -> T {
        try_! {
            let val = try_! { t? }?;
            val + 1
        }
    }
}
