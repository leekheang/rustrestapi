use actix_web::{web, HttpResponse , HttpRequest};
use crate::db_connection::{ PgPool, PgPooledConnection };

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

use crate::models::role::RoleList;
pub fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(RoleList::list(&pg_pool)))
 }

use crate::models::role::NewRole;
pub fn create(new_role: web::Json<NewRole>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_role
        .create(&pg_pool)
        .map(|role| HttpResponse::Ok().json(role))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

use crate::models::role::Role;
pub fn update(id: web::Path<i32>, new_role: web::Json<NewRole>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Role::update(&id, &new_role, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}


pub fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Role::find(&id, &pg_pool)
        .map(|role| HttpResponse::Ok().json(role))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}