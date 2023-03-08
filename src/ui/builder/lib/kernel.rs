#[derive(Debug,Clone)]
pub struct Kernel{
   pub(crate) version: String,
   pub(crate) url: Option<String>,
   pub(crate) path: Option<String>,
}