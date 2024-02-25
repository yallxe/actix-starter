mod injectable;
mod injector;

use crate::prelude::*;
use crate::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;

pub use self::injector::{Injector, DefaultInjector};
pub use self::injectable::Injectable;

pub(crate) struct Inject<T: Injectable + ?Sized, I: Injector<T> = DefaultInjector> {
    inner: Arc<T>,
    _phantom: PhantomData<I>,
}

impl<T: Injectable + ?Sized, I: Injector<T>> Deref for Inject<T, I> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Injectable + ?Sized, I: Injector<T>> FromRequest for Inject<T, I> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let instance = I::inject(req.clone());

        Box::pin(async move {
            Ok(Inject {
                inner: instance.await?,
                _phantom: PhantomData,
            })
        })
    }
}