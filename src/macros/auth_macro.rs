#[macro_export]
macro_rules! get_user_model {
    ($req:expr) => {
        async {
            let user_id = $req
                .extensions()
                .get::<UserId>()
                .map(|user_id| user_id.id)
                .unwrap();
            user_store::get_user(&user_id).await.unwrap()
        }
    };
}
