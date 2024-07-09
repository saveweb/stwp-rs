use crate::task::Id;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all(serialize = "lowercase"))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_json() {
        let item_id = ItemIdType::Int;
        let json = serde_json::to_string(&item_id).unwrap();
        assert_eq!(json, r#""int""#);
    }

    #[test]
    fn test_item_status_json() {
        let item_status = ItemStatusType::Int;
        let json = serde_json::to_string(&item_status).unwrap();
        assert_eq!(json, r#""int""#);
        let json = serde_json::to_string(&ItemStatusType::Str).unwrap();
        assert_eq!(json, r#""str""#);
        let json = serde_json::to_string(&ItemStatusType::None).unwrap();
        assert_eq!(json, r#""None""#);
    }
}
