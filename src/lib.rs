use biscotti::{Processor, RequestCookies, ResponseCookies, Key};
use std::{convert::Infallible, marker::PhantomData};
use axum::{extract::FromRequestParts, request::Parts, response::Response, extract::FromRef};

struct BiscottiJar<K = Key> {
	processor: RequestCookies,
	key: Key,
	_marker: PhantomData<K>
}



#[axum::async_trait]
impl<S, K> FromRequestParts<S> for BiscottiJar<K>
where
    S: Send + Sync,
    K: FromRef<S> + Into<Key>,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let k = K::from_ref(state);
        let key = k.into();
        let Self {
            jar,
            key,
            _marker: _,
        } = BiscottiJar::from_headers(&parts.headers, key);
        Ok(Self {
            jar,
            key,
            _marker: PhantomData,
        })
    }
}
