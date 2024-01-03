# Wishlist service for MiSArch

### Quickstart (Dev)

1. Open VSCode Development Container
2. `cargo run` starts GraphiQL on port 8000

### What it can do

- CRUD wishlists:
    ```rust
    pub struct Wishlist {
        pub id: String,
        pub user_id: String,
        pub product_variant_ids: Vec<String>,
        pub name: String,
        pub created_at: DateTime,
        pub last_updated_at: DateTime,
    }
    ```
- Validates all UUIDs input as strings
- Error prop to GraphQL