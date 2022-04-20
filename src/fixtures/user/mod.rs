use structiagram::diagram;

#[diagram]
pub struct User {
    pub id: models::UserId,
    pub name: String,
}
