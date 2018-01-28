use chrono::{DateTime, Utc};
use protobuf::well_known_types::Timestamp as ProtoTimestamp;
use protobuf::RepeatedField;

pub fn timestamp(datetime: DateTime<Utc>) -> ProtoTimestamp {
    let seconds = datetime.timestamp();
    let nanos = datetime.timestamp_subsec_nanos();
    let mut out = ProtoTimestamp::new();
    out.seconds = seconds;
    out.nanos = nanos as i32;
    out
}

pub fn fill_repeated<Proto: From<T>, T>(target: &mut RepeatedField<Proto>, existing: Vec<T>) {
    for element in existing.into_iter() {
        target.push(element.into())
    }
}
