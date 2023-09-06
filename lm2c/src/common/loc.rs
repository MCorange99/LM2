#[derive(Debug, Clone)]
pub struct Loc(
    pub String,
    pub u32,
    // pub u32
);

impl Loc {
    pub fn to_human(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}