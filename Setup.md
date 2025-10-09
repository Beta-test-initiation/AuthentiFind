### Setup database
1. Add location of your database in your `/env`
2. Run `sqlx database create`
3. Run `sqlx migrate add create_items_table`
4. Add your migration script @migrations/xyz.db
5. Run `sqlx migrate run`

