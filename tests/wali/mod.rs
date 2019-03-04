use std::path::Path;

pub struct case<T: Clone> {
    name: String,
    input: String,
    expected: String,
    parm: T,
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

pub struct wali<T: Clone> {
    config: test_config,
    case: Vec<case<T>>,
    test_fn: Fn(String, String, String, T) -> Result<(), String>,
}

impl<T: Clone> wali<T> {
    pub fn insert_case(&mut self, c: case<T>) {
        self.case.push(c);
    }

    pub fn verify(&self) {
        for case in &self.case {
            let mut input = self.config.input.clone();
            let mut expected = self.config.expected.clone();
            let mut output = self.config.output.clone();

            input.push_str(case.input.as_str());
            expected.push_str(case.expected.as_str());
            output.push_str(case.expected.as_str());

            match (self.test_fn)(input, output, expected, case.parm.clone()) {
                Ok(()) => {}
                Err(err) => assert!(false, err),
            }
        }
    }
}
