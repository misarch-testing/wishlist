use serde::Serialize;
use uuid::Uuid;

pub fn serialize_uuid<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    uuid.as_hyphenated().serialize(serializer)
}
