use actix_web::dev::{Service, Transform, ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::LocalBoxFuture;

pub struct CartItemMiddleware;

impl<S, B> Transform<S> for CartItemMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CartItemMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(CartItemMiddlewareService { service })
    }
}

pub struct CartItemMiddlewareService<S> {
    service: S,
}

impl<S, B> Service for CartItemMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let (req, res) = fut.await?.into_parts();
            let data = req.extensions_mut().remove::<CartItemMiddlewareData>();
            
            match data {
                // Further processing can go here, such as inserting data into Tera context or header.
                Some(data) => println!("Cart Items: {}", data.cart_item_count),
                None => println!("No cart items found"),
            }

            Ok(ServiceResponse::from_parts(req, res))
        })
    }
}