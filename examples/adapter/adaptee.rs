pub struct SpecificTarget;

impl SpecificTarget {
    pub fn specific_request(&self) -> String {
        "被适配对象 请求".into()
    }
}