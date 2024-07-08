pub enum ItemIdType {
    Str(String),
    Int(u64),
}

pub enum ItemStatus {
    None,
    Str(String),
    Int(u64),
}

pub struct Item {
    item_id: String,
    item_id_type: ItemIdType,
    item_status: String,
    item_status_type: ItemStatus,
    payload: String,
}
