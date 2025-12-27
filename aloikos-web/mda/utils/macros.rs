#[macro_export]
macro_rules! new_model {
    ($model:path, { $($field:ident : $value:expr),* $(,)? }) => {{
        use sea_orm::ActiveValue::Set;
        use sea_orm::Te
        $model {
            id: Set($crate::utils::snowflakes::new_id()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            $(
                $field: Set($value),
            )*
            ..Default::default()
        }
    }};
}

#[macro_export]
macro_rules! get_or_create {
    ($db:expr, $entity:ident, $id:expr, { $($field:ident : $value:expr),* $(,)? }) => {{
        use sea_orm::{
            EntityTrait, ActiveValue::Set, IntoActiveModel,
        };
        use chrono::Utc;

        match $entity::find_by_id($id.clone()).one($db).await {
            Ok(Some(existing)) => Ok(existing),
            Ok(None) => {
                let new_model = <$entity as EntityTrait>::Model {
                    id: $id.clone(),
                    $(
                        $field: $value.clone(),
                    )*
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    ..Default::default()
                };

                let active_model = new_model.into_active_model();
                match active_model.insert($db).await {
                    Ok(inserted) => Ok(inserted),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }};
}
