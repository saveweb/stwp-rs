use crate::task::Id;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all_fields(serialize = "lowercase"))]
pub enum ItemIdType {
    Int,
    Str,
}
impl From<&Id> for ItemIdType {
    fn from(s: &Id) -> Self {
        match s {
            Id::Int(_) => ItemIdType::Int,
            Id::Str(_) => ItemIdType::Str,
        }
    }
}



#[derive(Debug, Serialize, Clone)]
// #[serde(rename_all_fields(serialize = "lowercase"))]
pub enum ItemStatusType {
    None,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "str")]
    Str,
}

#[derive(Debug, Serialize, Clone)]
pub struct Item {
    pub item_id: String,
    pub item_id_type: String,
    pub item_status: String,
    pub item_status_type: ItemStatusType,
    pub payload: String,
}
