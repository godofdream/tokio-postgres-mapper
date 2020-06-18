use tokio_pg_mapper::PostgresMapper;
use postgres_types::ToSql;

#[derive(PostgresMapper, Debug, ToSql)]
#[pg_mapper(table = "user")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
}

fn main() {}
