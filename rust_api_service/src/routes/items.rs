use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{CreateItemRequest, Item, UpdateItemRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .service(get_items)
            .service(get_item)
            .service(create_item)
            .service(update_item)
            .service(delete_item),
    );
}

#[get("")]
async fn get_items(db_pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let items = sqlx::query(
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        ORDER BY id
        "#
    )
    .map(|row: SqliteRow| {
        Item {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    })
    .fetch_all(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(items))
}

#[get("/{id}")]
async fn get_item(
    db_pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();

    let item = sqlx::query(
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        WHERE id = $1
        "#
    )
    .bind(id)
    .map(|row: SqliteRow| {
        Item {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    })
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    match item {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        None => Err(ApiError::NotFoundError),
    }
}

#[post("")]
async fn create_item(
    db_pool: web::Data<DbPool>,
    item_data: web::Json<CreateItemRequest>,
) -> Result<impl Responder, ApiError> {
    let item = sqlx::query(
        r#"
        INSERT INTO items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#
    )
    .bind(&item_data.name)
    .bind(&item_data.description)
    .map(|row: SqliteRow| {
        Item {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    })
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(HttpResponse::Created().json(item))
}

#[put("/{id}")]
async fn update_item(
    db_pool: web::Data<DbPool>,
    path: web::Path<i32>,
    item_data: web::Json<UpdateItemRequest>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();

    // First check if the item exists
    let existing_item = sqlx::query(
        r#"
        SELECT id FROM items WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    if existing_item.is_none() {
        return Err(ApiError::NotFoundError);
    }

    // Update the item
    let now = Utc::now();
    let updated_item = sqlx::query(
        r#"
        UPDATE items
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            updated_at = $3
        WHERE id = $4
        RETURNING id, name, description, created_at, updated_at
        "#
    )
    .bind(&item_data.name)
    .bind(&item_data.description)
    .bind(now)
    .bind(id)
    .map(|row: SqliteRow| {
        Item {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    })
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(updated_item))
}

#[delete("/{id}")]
async fn delete_item(
    db_pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();

    // First check if the item exists
    let existing_item = sqlx::query(
        r#"
        SELECT id FROM items WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    if existing_item.is_none() {
        return Err(ApiError::NotFoundError);
    }

    // Delete the item
    sqlx::query(
        r#"
        DELETE FROM items
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(db_pool.get_ref())
    .await
    .map_err(ApiError::DatabaseError)?;

    Ok(HttpResponse::NoContent().finish())
}
