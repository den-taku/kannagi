use actix_web::Responder;

pub async fn root() -> impl Responder {
    "Root: Ok\n".to_string()
}

pub async fn test() -> String {
    "test: Ok\n".to_string()
}

pub async fn hello() -> impl Responder {
    "Hello!\n".to_string()
}
