use uuid::Uuid;
use super::schema::files;

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
}

#[derive(Queryable, Serialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
}
