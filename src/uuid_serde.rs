use bson::Uuid;
use serde::Serialize;

pub fn serialize_uuid<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    uuid.serialize(serializer)
}
