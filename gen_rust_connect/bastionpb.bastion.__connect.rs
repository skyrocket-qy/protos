///Shorthand for `OwnedView<CreateListingRequestView<'static>>`.
pub type OwnedCreateListingRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::CreateListingRequestView<'static>,
>;
///Shorthand for `OwnedView<CreateListingResponseView<'static>>`.
pub type OwnedCreateListingResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::CreateListingResponseView<'static>,
>;
///Shorthand for `OwnedView<BuyItemRequestView<'static>>`.
pub type OwnedBuyItemRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::BuyItemRequestView<'static>,
>;
///Shorthand for `OwnedView<BuyItemResponseView<'static>>`.
pub type OwnedBuyItemResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::BuyItemResponseView<'static>,
>;
///Shorthand for `OwnedView<ListActiveItemsRequestView<'static>>`.
pub type OwnedListActiveItemsRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsRequestView<'static>,
>;
///Shorthand for `OwnedView<ListActiveItemsResponseView<'static>>`.
pub type OwnedListActiveItemsResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsResponseView<'static>,
>;
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::CreateListingResponse>
for crate::gen_rust::bastionpb::__buffa::view::CreateListingResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::CreateListingResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::CreateListingResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::BuyItemResponse>
for crate::gen_rust::bastionpb::__buffa::view::BuyItemResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::BuyItemResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::BuyItemResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::ListActiveItemsResponse>
for crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::bastionpb::ListActiveItemsResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
/// Full service name for this service.
pub const AUCTION_SERVICE_SERVICE_NAME: &str = "bastionpb.AuctionService";
/// Static [`Spec`](::connectrpc::Spec) for the server-side `CreateListing` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUCTION_SERVICE_CREATE_LISTING_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/bastionpb.AuctionService/CreateListing",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `BuyItem` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUCTION_SERVICE_BUY_ITEM_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/bastionpb.AuctionService/BuyItem",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `ListActiveItems` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUCTION_SERVICE_LIST_ACTIVE_ITEMS_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/bastionpb.AuctionService/ListActiveItems",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Server trait for AuctionService.
///
/// # Implementing handlers
///
/// Handlers receive requests as `OwnedFooView` (an alias for
/// `OwnedView<FooView<'static>>`), which gives zero-copy borrowed access
/// to fields (e.g. `request.name` is a `&str` into the decoded buffer).
/// The view can be held across `.await` points. When two RPC types in
/// the same package would alias to the same `Owned<…>View` name (e.g.
/// a local message plus an imported one with the same short name), the
/// alias is suppressed for both and the request type is spelled as
/// `OwnedView<…View<'static>>` directly in the trait signature.
///
/// Implement methods with plain `async fn`; the returned future satisfies
/// the `Send` bound automatically. See the
/// [buffa user guide](https://github.com/anthropics/buffa/blob/main/docs/guide.md#ownedview-in-async-trait-implementations)
/// for zero-copy access patterns and when `to_owned_message()` is needed.
///
/// The `impl Encodable<Out>` return bound accepts the owned `Out`, the
/// generated `OutView<'_>` / `OwnedOutView`,
/// [`MaybeBorrowed`](::connectrpc::MaybeBorrowed), or
/// [`PreEncoded`](::connectrpc::PreEncoded) for handlers that encode a
/// non-`'static` view internally and pass the bytes across the handler
/// boundary. View bodies are not emitted for output types mapped via
/// `extern_path` (the impl would be an orphan); return owned for
/// WKT/extern outputs.
///
/// Server-streaming and bidi-streaming methods return
/// `ServiceStream<impl Encodable<Out> + Send + use<Self>>`. The
/// `use<Self>` precise-capturing clause excludes `&self`'s lifetime
/// (unary methods use `use<'a, Self>` and may borrow), so stream items
/// must be `'static`. To stream view-encoded data, encode each item
/// inside the stream body and yield
/// [`PreEncoded`](::connectrpc::PreEncoded) — see its `# Streaming
/// example` doc.
#[allow(clippy::type_complexity)]
pub trait AuctionService: Send + Sync + 'static {
    /// Handle the CreateListing RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn create_listing<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedCreateListingRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::bastionpb::CreateListingResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the BuyItem RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn buy_item<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedBuyItemRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::bastionpb::BuyItemResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the ListActiveItems RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn list_active_items<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedListActiveItemsRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::bastionpb::ListActiveItemsResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
}
/// Extension trait for registering a service implementation with a Router.
///
/// This trait is automatically implemented for all types that implement the service trait.
///
/// # Example
///
/// ```rust,ignore
/// use std::sync::Arc;
///
/// let service = Arc::new(MyServiceImpl);
/// let router = service.register(Router::new());
/// ```
pub trait AuctionServiceExt: AuctionService {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router;
}
impl<S: AuctionService> AuctionServiceExt for S {
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router {
        router
            .route_view(
                AUCTION_SERVICE_SERVICE_NAME,
                "CreateListing",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.create_listing(ctx, req)
                                .await?
                                .encode::<
                                    crate::gen_rust::bastionpb::CreateListingResponse,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(AUCTION_SERVICE_CREATE_LISTING_SPEC)
            .route_view(
                AUCTION_SERVICE_SERVICE_NAME,
                "BuyItem",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.buy_item(ctx, req)
                                .await?
                                .encode::<
                                    crate::gen_rust::bastionpb::BuyItemResponse,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(AUCTION_SERVICE_BUY_ITEM_SPEC)
            .route_view(
                AUCTION_SERVICE_SERVICE_NAME,
                "ListActiveItems",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.list_active_items(ctx, req)
                                .await?
                                .encode::<
                                    crate::gen_rust::bastionpb::ListActiveItemsResponse,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(AUCTION_SERVICE_LIST_ACTIVE_ITEMS_SPEC)
    }
}
/// Monomorphic dispatcher for `AuctionService`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = AuctionServiceServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct AuctionServiceServer<T> {
    inner: ::std::sync::Arc<T>,
}
impl<T: AuctionService> AuctionServiceServer<T> {
    /// Wrap a service implementation in a monomorphic dispatcher.
    pub fn new(service: T) -> Self {
        Self {
            inner: ::std::sync::Arc::new(service),
        }
    }
    /// Wrap an already-`Arc`'d service implementation.
    pub fn from_arc(inner: ::std::sync::Arc<T>) -> Self {
        Self { inner }
    }
}
impl<T> Clone for AuctionServiceServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(&self.inner),
        }
    }
}
impl<T: AuctionService> ::connectrpc::Dispatcher for AuctionServiceServer<T> {
    #[inline]
    fn lookup(
        &self,
        path: &str,
    ) -> Option<::connectrpc::dispatcher::codegen::MethodDescriptor> {
        let method = path.strip_prefix("bastionpb.AuctionService/")?;
        match method {
            "CreateListing" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUCTION_SERVICE_CREATE_LISTING_SPEC),
                )
            }
            "BuyItem" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUCTION_SERVICE_BUY_ITEM_SPEC),
                )
            }
            "ListActiveItems" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUCTION_SERVICE_LIST_ACTIVE_ITEMS_SPEC),
                )
            }
            _ => None,
        }
    }
    fn call_unary(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::Payload,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::UnaryResult {
        let Some(method) = path.strip_prefix("bastionpb.AuctionService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "CreateListing" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::bastionpb::__buffa::view::CreateListingRequestView,
                    >(request.encoded()?, format)?;
                    svc.create_listing(ctx, req)
                        .await?
                        .encode::<
                            crate::gen_rust::bastionpb::CreateListingResponse,
                        >(format)
                })
            }
            "BuyItem" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::bastionpb::__buffa::view::BuyItemRequestView,
                    >(request.encoded()?, format)?;
                    svc.buy_item(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::bastionpb::BuyItemResponse>(format)
                })
            }
            "ListActiveItems" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsRequestView,
                    >(request.encoded()?, format)?;
                    svc.list_active_items(ctx, req)
                        .await?
                        .encode::<
                            crate::gen_rust::bastionpb::ListActiveItemsResponse,
                        >(format)
                })
            }
            _ => ::connectrpc::dispatcher::codegen::unimplemented_unary(path),
        }
    }
    fn call_server_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        request: ::buffa::bytes::Bytes,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::StreamingResult {
        let Some(method) = path.strip_prefix("bastionpb.AuctionService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_streaming(path),
        }
    }
    fn call_client_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        requests: ::connectrpc::dispatcher::codegen::RequestStream,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::UnaryResult {
        let Some(method) = path.strip_prefix("bastionpb.AuctionService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_unary(path),
        }
    }
    fn call_bidi_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        requests: ::connectrpc::dispatcher::codegen::RequestStream,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::StreamingResult {
        let Some(method) = path.strip_prefix("bastionpb.AuctionService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_streaming(path),
        }
    }
}
/// Client for this service.
///
/// Generic over `T: ClientTransport`. For **gRPC** (HTTP/2), use
/// `Http2Connection` — it has honest `poll_ready` and composes with
/// `tower::balance` for multi-connection load balancing. For **Connect
/// over HTTP/1.1** (or unknown protocol), use `HttpClient`.
///
/// # Example (gRPC / HTTP/2)
///
/// ```rust,ignore
/// use connectrpc::client::{Http2Connection, ClientConfig};
/// use connectrpc::Protocol;
///
/// let uri: http::Uri = "http://localhost:8080".parse()?;
/// let conn = Http2Connection::connect_plaintext(uri.clone()).await?.shared(1024);
/// let config = ClientConfig::new(uri).with_protocol(Protocol::Grpc);
///
/// let client = AuctionServiceClient::new(conn, config);
/// let response = client.create_listing(request).await?;
/// ```
///
/// # Example (Connect / HTTP/1.1 or ALPN)
///
/// ```rust,ignore
/// use connectrpc::client::{HttpClient, ClientConfig};
///
/// let http = HttpClient::plaintext();  // cleartext http:// only
/// let config = ClientConfig::new("http://localhost:8080".parse()?);
///
/// let client = AuctionServiceClient::new(http, config);
/// let response = client.create_listing(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// The `OwnedView` derefs to the view, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.create_listing(request).await?.into_view();
/// let name: &str = resp.name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.create_listing(request).await?.into_owned();
/// ```
#[derive(Clone)]
pub struct AuctionServiceClient<T> {
    transport: T,
    config: ::connectrpc::client::ClientConfig,
}
impl<T> AuctionServiceClient<T>
where
    T: ::connectrpc::client::ClientTransport,
    <T::ResponseBody as ::http_body::Body>::Error: ::std::fmt::Display,
{
    /// Create a new client with the given transport and configuration.
    pub fn new(transport: T, config: ::connectrpc::client::ClientConfig) -> Self {
        Self { transport, config }
    }
    /// Get the client configuration.
    pub fn config(&self) -> &::connectrpc::client::ClientConfig {
        &self.config
    }
    /// Get a mutable reference to the client configuration.
    pub fn config_mut(&mut self) -> &mut ::connectrpc::client::ClientConfig {
        &mut self.config
    }
    /// Call the CreateListing RPC. Sends a request to /bastionpb.AuctionService/CreateListing.
    pub async fn create_listing(
        &self,
        request: crate::gen_rust::bastionpb::CreateListingRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::CreateListingResponseView<
                    'static,
                >,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.create_listing_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the CreateListing RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn create_listing_with_options(
        &self,
        request: crate::gen_rust::bastionpb::CreateListingRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::CreateListingResponseView<
                    'static,
                >,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUCTION_SERVICE_SERVICE_NAME,
                "CreateListing",
                request,
                options,
            )
            .await
    }
    /// Call the BuyItem RPC. Sends a request to /bastionpb.AuctionService/BuyItem.
    pub async fn buy_item(
        &self,
        request: crate::gen_rust::bastionpb::BuyItemRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::BuyItemResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.buy_item_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the BuyItem RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn buy_item_with_options(
        &self,
        request: crate::gen_rust::bastionpb::BuyItemRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::BuyItemResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUCTION_SERVICE_SERVICE_NAME,
                "BuyItem",
                request,
                options,
            )
            .await
    }
    /// Call the ListActiveItems RPC. Sends a request to /bastionpb.AuctionService/ListActiveItems.
    pub async fn list_active_items(
        &self,
        request: crate::gen_rust::bastionpb::ListActiveItemsRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsResponseView<
                    'static,
                >,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.list_active_items_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the ListActiveItems RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn list_active_items_with_options(
        &self,
        request: crate::gen_rust::bastionpb::ListActiveItemsRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::bastionpb::__buffa::view::ListActiveItemsResponseView<
                    'static,
                >,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUCTION_SERVICE_SERVICE_NAME,
                "ListActiveItems",
                request,
                options,
            )
            .await
    }
}
