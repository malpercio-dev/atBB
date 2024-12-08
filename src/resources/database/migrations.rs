use sqlx::Any;
use sqlx_migrator::Migration;

mod v1;

pub(crate) fn migrations() -> Vec<Box<dyn Migration<Any>>> {
    vec![Box::new(v1::V1Migration)]
}
