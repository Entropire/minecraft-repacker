pub trait PackTransform {
    fn from() -> PackFormat;
    fn to() -> PackFormat;
    
    fn apply(&self, pack: &mut serde_json::Value) -> Result<(), Box<dyn std::error::Error>>;
}