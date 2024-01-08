# Wishlist service for MiSArch

### Quickstart (Dev)

1. Open VSCode Development Container
2. `cargo run` starts GraphiQL on port 8000

### What it can do

- CRUD wishlists:
    ```rust
    pub struct Wishlist {
        pub id: Id,
        pub user_id: Id,
        pub product_variant_ids: HashSet<Id>,
        pub name: String,
        pub created_at: DateTime,
        pub last_updated_at: DateTime,
    }
    ```
- Validates all UUIDs input as strings
- Error prop to GraphQL