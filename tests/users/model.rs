extern crate fake;
extern crate planet_express;
use super::super::common;
use planet_express::core::auth::ViewerModel;
use planet_express::core::db::model::DatabaseModel;
use planet_express::user;

#[actix_rt::test]
async fn test_model_create() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::build();

    let record = user::User::create_user(obj.clone(), &db_conn)
        .await
        .unwrap();

    assert_eq!(obj.email, record.email);
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_model_get_by_id() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::create(db_conn.clone()).await;
    let record = user::User::read(obj.id, &db_conn).await.unwrap();

    assert_eq!(obj.email, record.email);
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_model_get_by_credentials() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::create(db_conn.clone()).await;
    let record = user::User::find_user_by_credentials(obj.email.clone(), obj.password, &db_conn)
        .await
        .unwrap();

    assert!(record.clone().is_some());
    assert_eq!(obj.email, record.unwrap().email);
    common::teardown(db_conn, test_name).await;
}
