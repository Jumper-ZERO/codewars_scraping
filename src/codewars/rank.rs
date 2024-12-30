use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub color: String,
}

impl Rank {
    pub fn snake_case(&self) -> String {
        match self.id {
            -8 => "kyu_8".to_string(),
            -7 => "kyu_7".to_string(),
            -6 => "kyu_6".to_string(),
            -5 => "kyu_5".to_string(),
            -4 => "kyu_4".to_string(),
            -3 => "kyu_3".to_string(),
            -2 => "kyu_2".to_string(),
            -1 => "kyu_1".to_string(),
            _ => "kyu_#".to_string()
        }
    }
}