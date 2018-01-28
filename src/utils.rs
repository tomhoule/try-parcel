use chrono::{DateTime, Utc};
use protobuf::well_known_types::Timestamp as ProtoTimestamp;

pub fn timestamp(datetime: DateTime<Utc>) -> ProtoTimestamp {
    let seconds = datetime.timestamp();
    let nanos = datetime.timestamp_subsec_nanos();
    let mut out = ProtoTimestamp::new();
    out.seconds = seconds;
    out.nanos = nanos as i32;
    out
}
