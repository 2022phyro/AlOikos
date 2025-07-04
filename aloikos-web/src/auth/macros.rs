#[macro_export]
macro_rules! new_model {
    ($model:path, { $($field:ident : $value:expr),* $(,)? }) => {{
        use sea_orm::ActiveValue::Set;
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
