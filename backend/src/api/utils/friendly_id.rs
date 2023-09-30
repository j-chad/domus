use uuid::Uuid;

pub enum ItemIdType {
    User,
}

impl ItemIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemIdType::User => "user",
        }
    }
}

pub trait ToFriendlyId {
    fn to_friendly_id(&self, item_type: ItemIdType) -> String;
}

impl ToFriendlyId for Uuid {
    fn to_friendly_id(&self, item_type: ItemIdType) -> String {
        let id = base62::encode(self.as_u128());
        format!("{}|{}", item_type.as_str(), id)
    }
}
