use serde::Serialize;

#[derive(Serialize)]
pub struct EquivalentSelection {
    pub id: i64,
    pub name: String,
    pub icon_thumbnail: String,
    pub is_selected: bool,
}

