#[macro_use]
pub mod macros;
mod cases;

pub struct Case<T: Clone + Sized + Default> {
    input: String,
    expected: String,
    param: T,
    is_corrupted: bool,
}

impl<T: Clone + Sized + Default> Default for Case<T> {
    #[inline(always)]
    fn default() -> Self {
        Case {
            expected: "".to_string(),
            input: "".to_string(),
            param: Default::default(),
            is_corrupted: false,
        }
    }
}

impl<T: Clone + Sized + Default> Case<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn corrupted(mut self) -> Self {
        self.is_corrupted = true;
        self
    }

    pub fn set_expected(mut self, expected: &'static str) -> Self {
        self.expected = expected.to_string();
        self
    }

    pub fn set_input(mut self, input: &'static str) -> Self {
        self.input = input.to_string();
        self
    }

    pub fn set_param(mut self, param: T) -> Self {
        self.param = param;
        self
    }
}

pub struct TestConfig {
    output: String,
    input: String,
    expected: String,
}

impl Default for TestConfig {
    #[inline(always)]
    fn default() -> Self {
        TestConfig {
            output: String::from("./_output/"),
            input: String::from("./dataset/images/"),
            expected: String::from("./dataset/reference/"),
        }
    }
}
