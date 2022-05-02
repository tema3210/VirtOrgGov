use actix_web::{HttpResponse, post, web};
use serde::{Deserialize,Serialize};
use uuid::Uuid;
use std::convert::TryInto;

#[derive(Deserialize)]
struct ChangeRequest {
    content: String,
    id: Uuid,
}

#[derive(Serialize)]
struct ChangeResponse {
    message: String,
    success: bool,
}

#[post("/api/changeLaw")]
#[tracing::instrument(skip_all)]
pub(crate) async fn law_change_endpoint(req: web::Json<ChangeRequest>) -> HttpResponse {
    tracing::info!("Law change triggered!");
    HttpResponse::build(400.try_into().unwrap())
        .json(ChangeResponse{message: format!("Asked to change {} law - no",req.id), success: false})
}