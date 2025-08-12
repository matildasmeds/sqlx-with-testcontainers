#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::eq;
    use sqlx::PgPool;
    use sqlx_tests::UserStorage;

    #[sqlx::test]
    async fn insert(pool: PgPool) {
        let users = UserStorage::new(pool);
        users.insert("Alice").await.unwrap();
        let count = users.count().await.unwrap();
        assert_that!(count, eq(1))
    }

    #[sqlx::test]
    async fn insert_again(pool: PgPool) {
        let users = UserStorage::new(pool);
        users.insert("Bob").await.unwrap();
        let count = users.count().await.unwrap();
        assert_that!(count, eq(1))
    }
}
