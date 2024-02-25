use crate::prelude::*;
use std::sync::Arc;
use actix_web::HttpRequest;
use crate::extractors::di::Injectable;

pub trait Injector<T: Injectable + ?Sized>: 'static {
    async fn inject(req: HttpRequest) -> Result<Arc<T>>;
}

pub struct DefaultInjector;

impl<T: Injectable + ?Sized> Injector<T> for DefaultInjector {
    async fn inject(req: HttpRequest) -> Result<Arc<T>> {
        T::get_instance(req).await
    }
}
