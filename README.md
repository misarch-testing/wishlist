# Wishlist service for MiSArch

### Quickstart (Dev)

1. Open VSCode Development Container
2. `cargo run` starts GraphiQL on port 8000

### What it can do

- CRUD wishlists:
    ```rust
    pub struct Wishlist {
        pub id: Uuid,
        pub user_id: Uuid,
        pub product_variants: HashSet<ProductVariant>,
        pub name: String,
        pub created_at: DateTime,
        pub last_updated_at: DateTime,
    }

    /// Foreign ProductVariant
    pub struct ProductVariant{
        id: Uuid
    }
    ```
- Validates all UUIDs input as strings
- Error prop to GraphQL