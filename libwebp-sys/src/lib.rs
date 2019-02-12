include!("./webp_bindings.rs");

pub const WEBP_ENCODER_ABI_VERSION: i32 = 526;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
