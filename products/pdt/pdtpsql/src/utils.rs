use std::ops::{Bound, Range};

use anyhow::Result;
use sqlx::{postgres::types::PgRange, query_as, PgPool};

#[derive(Debug)]
#[allow(dead_code)]
pub struct SchemaColumn {
    column_name: Option<String>,
    data_type: Option<String>,
}

pub async fn find_table(client: &PgPool, name: &str) -> Result<Option<Vec<SchemaColumn>>> {
    let schema = query_as!(SchemaColumn, "SELECT column_name, data_type FROM information_schema.columns where table_name=$1 order by ordinal_position", name).fetch_all(client).await?;
    if schema.is_empty() {
        return Ok(None);
    }
    Ok(Some(schema))
}

pub fn pg_to_std_range<T: Default>(pg_range: PgRange<T>) -> Range<T> {
    // this is valid so long as we agree to always use half-open [) bounds in psql
    let start = match pg_range.start {
        Bound::Included(value) => value,
        Bound::Excluded(value) => value,
        Bound::Unbounded => Default::default(),
    };

    let end = match pg_range.end {
        Bound::Included(value) => value,
        Bound::Excluded(value) => value,
        Bound::Unbounded => Default::default(),
    };

    start..end
}
