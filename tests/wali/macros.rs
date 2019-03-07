#[macro_export]
macro_rules! wali_test {
    ($name: ident, $func: ident,$case:expr,$config:expr) => {
        #[test]
        fn $name() {
            match $func($config, $case) {
                Ok(()) => {}
                Err(err) => assert!(false, err),
            }
        }
    };

    ($name: ident, $func: ident,$case:expr) => {
        #[test]
        fn $name() {
            match $func(&Default::default(), $case) {
                Ok(()) => {}
                Err(err) => assert!(false, err),
            }
        }
    };
}
