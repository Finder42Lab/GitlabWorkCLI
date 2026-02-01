use std::borrow::Cow;
use derive_builder::Builder;
use gitlab::api::Endpoint;
use gitlab::api::endpoint_prelude::Method;

#[derive(Debug, Builder, Clone)]
pub struct EpicApi {
    group_id: u64,
    iid: u16
}

impl EpicApi {
    /// Create a builder for the endpoint.
    pub fn builder() -> EpicApiBuilder {
        EpicApiBuilder::default()
    }
}

impl Endpoint for EpicApi {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/epics/{}", self.group_id, self.iid).into()
    }
}