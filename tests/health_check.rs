

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expext("Failed to spawn our app")
}
