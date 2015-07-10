use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Pattern {
    name: Option<String>,
    uuid: Uuid,
}

impl Pattern {
    pub fn new(uuid: Uuid) -> Pattern {
        Pattern{
            uuid: uuid,
            name: None
        }
    }
}
