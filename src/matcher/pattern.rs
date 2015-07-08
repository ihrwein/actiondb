use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
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
