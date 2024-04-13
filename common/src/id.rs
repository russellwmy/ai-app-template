use uuid::Uuid;

pub fn generate_id_with_data(data: &str) -> String {
    uuid::Uuid::new_v5(&Uuid::NAMESPACE_OID, data.as_bytes()).to_string()
}

pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
