use std::path::Path;

mod cases;

pub struct case<T: Clone + Sized> {
    input: String,
    expected: String,
    param: T,
    is_corrupted: bool,
}

pub struct test_config {
    output: String,
    input: String,
    expected: String,
}

impl Default for test_config {
    #[inline(always)]
    fn default() -> Self {
        test_config {
            output: String::from("./_output/"),
            input: String::from("./dataset/images/"),
            expected: String::from("./dataset/reference/"),
        }
    }
}

pub struct wali<T: Clone + Sized> {
    config: test_config,
    case: Vec<case<T>>,
    test_fn: Box<dyn Fn(&test_config, String, String, bool, T) -> Result<(), String>>,
}

impl<T: Clone + Sized> wali<T> {
    pub fn insert_case(mut self, c: case<T>) -> Self {
        self.case.push(c);
        self
    }

    pub fn verify(&self) {
        for case in &self.case {
            let input = case.input.clone();
            let expected = case.expected.clone();
            let is_corrupted = case.is_corrupted;
            match (self.test_fn)(
                &self.config,
                input,
                expected,
                is_corrupted,
                case.param.clone(),
            ) {
                Ok(()) => {}
                Err(err) => assert!(false, err),
            }
        }
    }
}
