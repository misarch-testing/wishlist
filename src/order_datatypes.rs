use async_graphql::{Enum, InputObject, SimpleObject};

/// GraphQL order direction.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderDirection {
    /// Ascending order direction.
    Asc,
    /// Descending order direction.
    Desc,
}

impl Default for OrderDirection {
    fn default() -> Self {
        Self::Asc
    }
}

/// Implements conversion to i32 for MongoDB document sorting.
impl From<OrderDirection> for i32 {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Asc => 1,
            OrderDirection::Desc => -1,
        }
    }
}

/// Describes the fields that a wishlist can be ordered by.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum WishlistOrderField {
    /// Orders by "id".
    Id,
    /// Orders by "user_id".
    UserId,
    /// Orders by "name".
    Name,
    /// Orders by "created_at".
    CreatedAt,
    /// Orders by "last_updated_at".
    LastUpdatedAt,
}

impl WishlistOrderField {
    pub fn as_str(&self) -> &'static str {
        match self {
            WishlistOrderField::Id => "_id",
            WishlistOrderField::UserId => "user_id",
            WishlistOrderField::Name => "name",
            WishlistOrderField::CreatedAt => "created_at",
            WishlistOrderField::LastUpdatedAt => "last_updated_at",
        }
    }
}

impl Default for WishlistOrderField {
    fn default() -> Self {
        Self::Id
    }
}

/// Specifies the order of wishlists.
#[derive(SimpleObject, InputObject)]
pub struct WishlistOrder {
    /// Order direction of wishlists.
    pub direction: Option<OrderDirection>,
    /// Field that wishlists should be ordered by.
    pub field: Option<WishlistOrderField>,
}

impl Default for WishlistOrder {
    fn default() -> Self {
        Self {
            direction: Some(Default::default()),
            field: Some(Default::default()),
        }
    }
}
