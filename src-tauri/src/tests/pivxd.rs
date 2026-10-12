use crate::tests::utils::connect;

#[tokio::test]
async fn setups_pivxd_docker() {
    let mut conn = connect();
    conn.download_dependencies().await.unwrap();
    conn.setup_pivxd_docker().await.unwrap();
}
