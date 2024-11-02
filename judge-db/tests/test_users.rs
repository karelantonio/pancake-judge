use judge_db::DatabaseService;

#[test]
fn test_basic_usage() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let db = DatabaseService::new("sqlite://:memory:").await.unwrap();
        db.run_migrations().await.unwrap();

        let users = db.users();
        let all_users = users.query_all_users().await.unwrap();
        // Should contain only the administrator
        assert_eq!(all_users.len(), 1);
        let fst = all_users.first().unwrap();
        assert_eq!(fst.username, "root");
    });
}
