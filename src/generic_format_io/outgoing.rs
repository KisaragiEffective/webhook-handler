use serde::Serialize;
use std::marker::PhantomData;

pub struct GenericOutgoingSerializer<S: Serialize, F: FnOnce(S) -> &'static str> {
    f: F,
    __phantom: PhantomData<S>
}

impl <S: Serialize, F: FnOnce(S) -> &'static str> GenericOutgoingSerializer<S, F> {
    pub(crate) fn new(f: F) -> Self {
        GenericOutgoingSerializer {
            f,
            __phantom: PhantomData
        }
    }
}
