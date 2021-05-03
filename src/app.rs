#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App {
    pub id: i32,
    pub apptype: String,
    pub version: String,
    pub name: String,
    pub icon: Option<Vec<u8>>,
}

impl App {
    pub async fn fetch_icon(&self) -> Result<Vec<u8>, String> {
        // self.icon = ...
        todo!()
    }
}