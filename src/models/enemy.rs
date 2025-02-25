pub struct Intent {
    pub damage: u32,
    pub healing: u32,
    pub status: Vec<String>,
}
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub actions: Vec<Intent>,
}