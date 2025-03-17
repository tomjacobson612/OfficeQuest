pub enum Intent {
    Damage {amount: u32},
    Healing {amount: u32},
    Status,
}
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub actions: Vec<Intent>,
}

impl Enemy {
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn hr_rep() -> Enemy {
        Enemy { 
            name: "HR Rep".to_string(), 
            hp: 5, 
            actions: vec![Intent::Damage {amount: 1}, Intent::Healing{amount: 1}]
        }
    }
}
