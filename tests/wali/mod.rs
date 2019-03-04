use std::path::Path;

pub struct case {
    name: String,
    input: String,
    expected: String,
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

pub struct wali {
    config: test_config,
    case: Vec<case>,
    test_fn: Fn(String, String, String) -> Result<(), String>,
}

impl wali {
    pub fn insert_case(&mut self, c: case) {
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
        }
    }
}
