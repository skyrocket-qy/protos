///Shorthand for `OwnedView<TupleView<'static>>`.
pub type OwnedTupleView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::TupleView<'static>,
>;
///Shorthand for `OwnedView<EmptyView<'static>>`.
pub type OwnedEmptyView = ::buffa::view::OwnedView<
    ::buffa_types::google::protobuf::__buffa::view::EmptyView<'static>,
>;
///Shorthand for `OwnedView<ListTuplesInView<'static>>`.
pub type OwnedListTuplesInView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::ListTuplesInView<'static>,
>;
///Shorthand for `OwnedView<ListTuplesOutView<'static>>`.
pub type OwnedListTuplesOutView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::ListTuplesOutView<'static>,
>;
///Shorthand for `OwnedView<DeleteTuplesInView<'static>>`.
pub type OwnedDeleteTuplesInView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::DeleteTuplesInView<'static>,
>;
///Shorthand for `OwnedView<CheckInView<'static>>`.
pub type OwnedCheckInView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::CheckInView<'static>,
>;
///Shorthand for `OwnedView<CheckOutView<'static>>`.
pub type OwnedCheckOutView = ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::CheckOutView<'static>,
>;
impl ::connectrpc::Encodable<crate::gen_rust::authzpb::ListTuplesOut>
for crate::gen_rust::authzpb::__buffa::view::ListTuplesOutView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::authzpb::ListTuplesOut>
for ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::ListTuplesOutView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::authzpb::CheckOut>
for crate::gen_rust::authzpb::__buffa::view::CheckOutView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::authzpb::CheckOut>
for ::buffa::view::OwnedView<
    crate::gen_rust::authzpb::__buffa::view::CheckOutView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
/// Full service name for this service.
pub const AUTHZ_SERVICE_SERVICE_NAME: &str = "authzpb.AuthzService";
/// Static [`Spec`](::connectrpc::Spec) for the server-side `CreateTuple` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUTHZ_SERVICE_CREATE_TUPLE_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/authzpb.AuthzService/CreateTuple",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `ListTuples` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUTHZ_SERVICE_LIST_TUPLES_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/authzpb.AuthzService/ListTuples",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `DeleteTuples` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUTHZ_SERVICE_DELETE_TUPLES_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/authzpb.AuthzService/DeleteTuples",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `Check` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const AUTHZ_SERVICE_CHECK_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/authzpb.AuthzService/Check",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Server trait for AuthzService.
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
pub trait AuthzService: Send + Sync + 'static {
    /// Handle the CreateTuple RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn create_tuple<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedTupleView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                ::buffa_types::google::protobuf::Empty,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the ListTuples RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn list_tuples<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedListTuplesInView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::authzpb::ListTuplesOut,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the DeleteTuples RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn delete_tuples<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedDeleteTuplesInView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                ::buffa_types::google::protobuf::Empty,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the Check RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn check<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedCheckInView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::authzpb::CheckOut,
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
pub trait AuthzServiceExt: AuthzService {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router;
}
impl<S: AuthzService> AuthzServiceExt for S {
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router {
        router
            .route_view(
                AUTHZ_SERVICE_SERVICE_NAME,
                "CreateTuple",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.create_tuple(ctx, req)
                                .await?
                                .encode::<::buffa_types::google::protobuf::Empty>(format)
                        }
                    })
                },
            )
            .with_spec(AUTHZ_SERVICE_CREATE_TUPLE_SPEC)
            .route_view(
                AUTHZ_SERVICE_SERVICE_NAME,
                "ListTuples",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.list_tuples(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::authzpb::ListTuplesOut>(format)
                        }
                    })
                },
            )
            .with_spec(AUTHZ_SERVICE_LIST_TUPLES_SPEC)
            .route_view(
                AUTHZ_SERVICE_SERVICE_NAME,
                "DeleteTuples",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.delete_tuples(ctx, req)
                                .await?
                                .encode::<::buffa_types::google::protobuf::Empty>(format)
                        }
                    })
                },
            )
            .with_spec(AUTHZ_SERVICE_DELETE_TUPLES_SPEC)
            .route_view(
                AUTHZ_SERVICE_SERVICE_NAME,
                "Check",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.check(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::authzpb::CheckOut>(format)
                        }
                    })
                },
            )
            .with_spec(AUTHZ_SERVICE_CHECK_SPEC)
    }
}
/// Monomorphic dispatcher for `AuthzService`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = AuthzServiceServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct AuthzServiceServer<T> {
    inner: ::std::sync::Arc<T>,
}
impl<T: AuthzService> AuthzServiceServer<T> {
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
impl<T> Clone for AuthzServiceServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(&self.inner),
        }
    }
}
impl<T: AuthzService> ::connectrpc::Dispatcher for AuthzServiceServer<T> {
    #[inline]
    fn lookup(
        &self,
        path: &str,
    ) -> Option<::connectrpc::dispatcher::codegen::MethodDescriptor> {
        let method = path.strip_prefix("authzpb.AuthzService/")?;
        match method {
            "CreateTuple" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUTHZ_SERVICE_CREATE_TUPLE_SPEC),
                )
            }
            "ListTuples" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUTHZ_SERVICE_LIST_TUPLES_SPEC),
                )
            }
            "DeleteTuples" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUTHZ_SERVICE_DELETE_TUPLES_SPEC),
                )
            }
            "Check" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(AUTHZ_SERVICE_CHECK_SPEC),
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
        let Some(method) = path.strip_prefix("authzpb.AuthzService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "CreateTuple" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::authzpb::__buffa::view::TupleView,
                    >(request.encoded()?, format)?;
                    svc.create_tuple(ctx, req)
                        .await?
                        .encode::<::buffa_types::google::protobuf::Empty>(format)
                })
            }
            "ListTuples" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::authzpb::__buffa::view::ListTuplesInView,
                    >(request.encoded()?, format)?;
                    svc.list_tuples(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::authzpb::ListTuplesOut>(format)
                })
            }
            "DeleteTuples" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::authzpb::__buffa::view::DeleteTuplesInView,
                    >(request.encoded()?, format)?;
                    svc.delete_tuples(ctx, req)
                        .await?
                        .encode::<::buffa_types::google::protobuf::Empty>(format)
                })
            }
            "Check" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::authzpb::__buffa::view::CheckInView,
                    >(request.encoded()?, format)?;
                    svc.check(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::authzpb::CheckOut>(format)
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
        let Some(method) = path.strip_prefix("authzpb.AuthzService/") else {
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
        let Some(method) = path.strip_prefix("authzpb.AuthzService/") else {
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
        let Some(method) = path.strip_prefix("authzpb.AuthzService/") else {
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
/// let client = AuthzServiceClient::new(conn, config);
/// let response = client.create_tuple(request).await?;
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
/// let client = AuthzServiceClient::new(http, config);
/// let response = client.create_tuple(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// The `OwnedView` derefs to the view, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.create_tuple(request).await?.into_view();
/// let name: &str = resp.name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.create_tuple(request).await?.into_owned();
/// ```
#[derive(Clone)]
pub struct AuthzServiceClient<T> {
    transport: T,
    config: ::connectrpc::client::ClientConfig,
}
impl<T> AuthzServiceClient<T>
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
    /// Call the CreateTuple RPC. Sends a request to /authzpb.AuthzService/CreateTuple.
    pub async fn create_tuple(
        &self,
        request: crate::gen_rust::authzpb::Tuple,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                ::buffa_types::google::protobuf::__buffa::view::EmptyView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.create_tuple_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the CreateTuple RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn create_tuple_with_options(
        &self,
        request: crate::gen_rust::authzpb::Tuple,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                ::buffa_types::google::protobuf::__buffa::view::EmptyView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUTHZ_SERVICE_SERVICE_NAME,
                "CreateTuple",
                request,
                options,
            )
            .await
    }
    /// Call the ListTuples RPC. Sends a request to /authzpb.AuthzService/ListTuples.
    pub async fn list_tuples(
        &self,
        request: crate::gen_rust::authzpb::ListTuplesIn,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::authzpb::__buffa::view::ListTuplesOutView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.list_tuples_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the ListTuples RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn list_tuples_with_options(
        &self,
        request: crate::gen_rust::authzpb::ListTuplesIn,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::authzpb::__buffa::view::ListTuplesOutView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUTHZ_SERVICE_SERVICE_NAME,
                "ListTuples",
                request,
                options,
            )
            .await
    }
    /// Call the DeleteTuples RPC. Sends a request to /authzpb.AuthzService/DeleteTuples.
    pub async fn delete_tuples(
        &self,
        request: crate::gen_rust::authzpb::DeleteTuplesIn,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                ::buffa_types::google::protobuf::__buffa::view::EmptyView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.delete_tuples_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the DeleteTuples RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn delete_tuples_with_options(
        &self,
        request: crate::gen_rust::authzpb::DeleteTuplesIn,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                ::buffa_types::google::protobuf::__buffa::view::EmptyView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUTHZ_SERVICE_SERVICE_NAME,
                "DeleteTuples",
                request,
                options,
            )
            .await
    }
    /// Call the Check RPC. Sends a request to /authzpb.AuthzService/Check.
    pub async fn check(
        &self,
        request: crate::gen_rust::authzpb::CheckIn,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::authzpb::__buffa::view::CheckOutView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.check_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the Check RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn check_with_options(
        &self,
        request: crate::gen_rust::authzpb::CheckIn,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::authzpb::__buffa::view::CheckOutView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                AUTHZ_SERVICE_SERVICE_NAME,
                "Check",
                request,
                options,
            )
            .await
    }
}
