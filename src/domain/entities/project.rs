use mongodb::bson::oid::ObjectId; // Temporary, will be removed later if not needed

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Option<ObjectId>, // Or String, depending on how it's identified in the domain
    pub name: String,
    pub number: i32,
    // Add other relevant fields
}
