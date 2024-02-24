mod injectable;

use crate::prelude::*;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::error::Error;
use crate::extractors::di::injectable::Injectable;

pub(crate) struct Inject<T: Injectable + ?Sized>(Arc<T>);

impl<T: Injectable + ?Sized> Deref for Inject<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Injectable + ?Sized> FromRequest for Inject<T> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let instance = T::get_instance(req.clone());

        Box::pin(async move {
            Ok(Inject(instance.await?))
        })
    }
}