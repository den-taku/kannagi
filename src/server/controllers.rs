use actix_web::Responder;

pub async fn sound_test_wav() -> impl Responder {
    crate::devices::audio::play_test_wav();
    "Ok\n".to_string()
}

pub async fn sound_test_m4a() -> impl Responder {
    crate::devices::audio::play_test_m4a();
    "Ok\n".to_string()
}

pub async fn root() -> impl Responder {
    "Root: Ok\n".to_string()
}

pub async fn test() -> String {
    "test: Ok\n".to_string()
}

pub async fn hello() -> impl Responder {
    "Hello!\n".to_string()
}
