use actix_web::{Responder, get, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use crate::templates as temp;
use crate::schema as db;
use crate::orm as orm;
use diesel::{QueryDsl, RunQueryDsl, PgConnection, ExpressionMethods, NullableExpressionMethods, GroupByDsl};
use actix_web::http::StatusCode;
use diesel::query_dsl::LoadQuery;
use std::convert::TryInto;

fn get_law_data_query(offset: usize, limit: usize)  -> String {
    //safety
    let limit = limit.clamp(1,50);
    let sql = format!("\
            select what,new_text,submit_date,status from law_data inner join (
	            select what as w, MAX(submit_date) as sd from law_data group by what
            ) a on law_data.what = a.w and law_data.submit_date = a.sd LIMIT {} OFFSET {};
        \
        ",limit,offset);
    sql
}

#[get("/")]
#[tracing::instrument(skip_all)]
pub(crate) async fn index(pool: web::Data<crate::DbPool>) -> impl Responder {
    let laws = |conn: &mut PgConnection| -> Result<Vec<orm::LawData>,_> {
        tracing::trace!("index: law data fetch...");

        let sql = get_law_data_query(0,5);

        diesel::sql_query(sql).internal_load(conn)
    };
    let parties = |conn: &mut PgConnection| -> Result<Vec<orm::Party>,_> {
        tracing::trace!("index: parties data fetch...");
        use db::parties::dsl::*;
        parties
            .load::<orm::Party>(conn)
    };
    let codexes = |conn: &mut PgConnection| -> Result<Vec<orm::Codex>,_> {
        tracing::trace!("index: codexes data fetch...");
        use db::codexes::dsl::*;
        codexes
            .load::<orm::Codex>(conn)
    };

    if let Ok(conn) = pool.get().await {
        tracing::info!("Pool get is ok");

        let laws = conn.interact(laws).await;
        let parties = conn.interact(parties).await;
        let codexes = conn.interact(codexes).await;

        if matches!(laws,Ok(Ok(_))) {
            tracing::trace!("laws req. - success");
        } else {
            tracing::warn!("laws req. - failure: {:?}",&laws);
        };
        if matches!(parties,Ok(Ok(_))) {
            tracing::trace!("parties req. - success");
        } else {
            tracing::warn!("parties req. - failure: {:?}",&parties);
        };
        if matches!(codexes,Ok(Ok(_))) {
            tracing::trace!("codexes req. - success");
        } else {
            tracing::warn!("codexes req. - failure: {:?}",&codexes);
        };

        match (laws,parties,codexes) {
            (Ok(Ok(laws)),Ok(Ok(parties)),Ok(Ok(codexes))) => {
                //Ok branch
                tracing::info!("Rendering an index");
                let page = temp::Index{laws,parties,codexes};
                Ok(page)
            },
            _ => {
                //other 26 error branches
                Err(())
            }
        }
    } else {
        //pool error
        tracing::warn!("Pool get failed");
        Err(())
    }.map_err(|_| actix_web::error::InternalError::new("Pool or ORM have failed",StatusCode::INTERNAL_SERVER_ERROR))
}

#[derive(Deserialize)]
pub struct ChangeLawForm {
    id: String,
}

#[get("/changeLaw")]
#[tracing::instrument(skip_all)]
pub(crate) async fn change_law_page(form: web::Query<ChangeLawForm>,_pool: web::Data<crate::DbPool>) -> HttpResponse {
    let data = Uuid::parse_str(&form.id).map_err(
        |_| actix_web::error::InternalError::new("Bad UUID",StatusCode::BAD_REQUEST)
    ).map(
        |uuid| {
            tracing::info!("Attempt to change law {} blocked",&uuid);
            temp::ChangeLawPage{ id: uuid }
        }
    );
    let mut resp = HttpResponse::build(if data.is_ok() { 200 } else { 500 }.try_into().unwrap());
    match data {
        Ok(data) => {
            let mut resp = askama_actix::TemplateToResponse::to_response(&data);
            resp.head_mut().headers_mut()
                .insert(
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::HeaderValue::from_static("text/html")
                );
            resp
        },
        Err(e) => {
            resp.content_type("text/plain");
            resp.body(format!("{}",e))
        }
    }
}

#[derive(Deserialize)]
pub struct LawsForm {
    page_size: usize,
    offset: usize,
}

#[get("/laws")]
#[tracing::instrument(skip_all)]
pub(crate) async fn laws_page(form: web::Query<LawsForm>,pool: web::Data<crate::DbPool>) -> impl Responder {

    let laws = move |conn: &mut PgConnection| -> Result<Vec<orm::LawData>,_> {
        let sql = get_law_data_query(form.offset,form.page_size);
        diesel::sql_query(sql).internal_load(conn)
    };
    let obj = pool.get().await;
    match obj {
      Ok(f) => {
        match f.interact(laws).await {
            Ok(Ok(data)) => {
                //???
                tracing::info!("Rendering laws page");
                Ok(temp::LawsPage {laws: data})
            },
            _ => {
                Err(actix_web::error::InternalError::new("Pool or ORM have failed",StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
      },
      Err(_) => {
        Err(actix_web::error::InternalError::new("Pool or ORM have failed",StatusCode::INTERNAL_SERVER_ERROR))
      }
    }
}

#[get("/law/{id}")]
#[tracing::instrument(skip_all)]
pub(crate) async fn law_page(path: web::Path<Uuid>,pool: web::Data<crate::DbPool>) -> impl Responder {
    "unimplemented"
}

#[get("/party/{id}")]
#[tracing::instrument(skip_all)]
pub(crate) async fn party_page(path: web::Path<Uuid>,pool: web::Data<crate::DbPool>) -> impl Responder {
    "unimplemented"
}

#[get("/codex/{id}")]
#[tracing::instrument(skip_all)]
pub(crate) async fn codex_page(path: web::Path<Uuid>,pool: web::Data<crate::DbPool>) -> impl Responder {
    "unimplemented"
}

#[get("/politic/{id}")]
#[tracing::instrument(skip_all)]
pub(crate) async fn politic_page(path: web::Path<Uuid>,pool: web::Data<crate::DbPool>) -> impl Responder {
    "unimplemented"
}