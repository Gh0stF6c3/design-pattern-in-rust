pub trait Target {
    fn request(&self) -> String;
}

pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "接收请求".into()
    }
}

pub fn call(target: impl Target) {
    println!("'{}'", target.request());
}