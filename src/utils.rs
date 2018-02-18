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

// Needs to implement QueryFragment
pub enum Constraint<T: ::std::str::FromStr> {
    Eq(T),
    Neq(T),
    Like(String),
    Sort,
}

impl<'a, T: ::std::str::FromStr> ::std::str::FromStr for Constraint<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Constraint<T>, ()> {
        unimplemented!();
    }
}

/// Takes a table name, a list of fields to allow filtering by, and produces a struct implementing QueryFragment, so it can be used in a where clause, as well as From<$proto>.
macro_rules! filters_for_table {
    (
        $name:ident,
        $proto:ty,
        $table:path,
        $($field:ident : $type:ty),*
    ) => {

        pub struct $name {
            *(pub $field: Option<Constraint<$type>>),
        }

        impl $name {
            pub fn filter(&self) -> <$table as BoxedDsl>::Output {
                let q = $table.into_boxed();

                *(
                    if let Some(constraint) = self.$field {
                        match constraint {
                            _ => unimplemented!()
                        }
                    }
                )
            }
        }

        // impl ::diesel::query_dsl::QueryFragment {
        //     fn walk_ast(&self, pass: AstPass<Pg>) -> QueryResult<()> {
        //         *(
        //             // match on the operator, then
        //             // $field.op(val).walk_ast(pass)
        //             pass.push_sql("$field ")
        //             let constraint = Constraint<T>::from_str(&self.$field).unwrap();
        //             constraint.walk_ast(pass)?;
        //             pass.push_sql(" AND ");
        //         )
        //         pass.push_sql("1 = 1");
        //     }
        // }

        impl From<$proto> for $name {
            fn from(proto: $proto) -> $name {
                $name {
                    *($field: proto.$field),
                }
            }
        }
    }
}
