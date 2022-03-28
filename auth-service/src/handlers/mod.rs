use actix_web:: {web, web::ServiceConfig, HttpResponse};

pub fn app_config(config: &mut ServiceConfig) {
    //getting some kind of http request into health
    let health_resource = web::resource("/")
        .route(web::get().to(health));

    config.service(health_resource);
}

// this function returns HttpResponse
pub async fn health()-> HttpResponse {
    HttpResponse::Ok().finish()
}