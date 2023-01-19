use common::sea_orm::{ActiveModelBehavior, DeleteResult};

pub async fn delete_enitity<E>(
    db_connection: &common::sea_orm::DatabaseConnection,
    entity: E,
) -> Result<DeleteResult, String>
where
    E: ActiveModelBehavior + std::marker::Send,
{
    match common::sea_orm::ActiveModelTrait::delete(entity, db_connection).await {
        Ok(val) => Ok(val),
        Err(e) => Err(e.to_string()),
    }
}
