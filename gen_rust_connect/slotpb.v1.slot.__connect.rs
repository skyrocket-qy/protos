///Shorthand for `OwnedView<PlayReqView<'static>>`.
pub type OwnedPlayReqView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::PlayReqView<'static>,
>;
///Shorthand for `OwnedView<PlayRespView<'static>>`.
pub type OwnedPlayRespView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::PlayRespView<'static>,
>;
///Shorthand for `OwnedView<BuyFgReqView<'static>>`.
pub type OwnedBuyFgReqView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::BuyFgReqView<'static>,
>;
///Shorthand for `OwnedView<BuyFgRespView<'static>>`.
pub type OwnedBuyFgRespView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::BuyFgRespView<'static>,
>;
///Shorthand for `OwnedView<GetRoomInfoReqView<'static>>`.
pub type OwnedGetRoomInfoReqView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoReqView<'static>,
>;
///Shorthand for `OwnedView<GetRoomInfoRespView<'static>>`.
pub type OwnedGetRoomInfoRespView = ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoRespView<'static>,
>;
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::PlayResp>
for crate::gen_rust::slotpb::v1::__buffa::view::PlayRespView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::PlayResp>
for ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::PlayRespView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::BuyFgResp>
for crate::gen_rust::slotpb::v1::__buffa::view::BuyFgRespView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::BuyFgResp>
for ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::BuyFgRespView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::GetRoomInfoResp>
for crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoRespView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::slotpb::v1::GetRoomInfoResp>
for ::buffa::view::OwnedView<
    crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoRespView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
/// Full service name for this service.
pub const SLOT_SERVICE_SERVICE_NAME: &str = "slotpb.v1.SlotService";
/// Static [`Spec`](::connectrpc::Spec) for the server-side `Play` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const SLOT_SERVICE_PLAY_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/slotpb.v1.SlotService/Play",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `BuyFg` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const SLOT_SERVICE_BUY_FG_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/slotpb.v1.SlotService/BuyFg",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `GetRoomInfo` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const SLOT_SERVICE_GET_ROOM_INFO_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/slotpb.v1.SlotService/GetRoomInfo",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Server trait for SlotService.
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
pub trait SlotService: Send + Sync + 'static {
    /// Handle the Play RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn play<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedPlayReqView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::slotpb::v1::PlayResp,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the BuyFg RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn buy_fg<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedBuyFgReqView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::slotpb::v1::BuyFgResp,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the GetRoomInfo RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn get_room_info<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedGetRoomInfoReqView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::slotpb::v1::GetRoomInfoResp,
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
pub trait SlotServiceExt: SlotService {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router;
}
impl<S: SlotService> SlotServiceExt for S {
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router {
        router
            .route_view(
                SLOT_SERVICE_SERVICE_NAME,
                "Play",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.play(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::slotpb::v1::PlayResp>(format)
                        }
                    })
                },
            )
            .with_spec(SLOT_SERVICE_PLAY_SPEC)
            .route_view(
                SLOT_SERVICE_SERVICE_NAME,
                "BuyFg",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.buy_fg(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::slotpb::v1::BuyFgResp>(format)
                        }
                    })
                },
            )
            .with_spec(SLOT_SERVICE_BUY_FG_SPEC)
            .route_view(
                SLOT_SERVICE_SERVICE_NAME,
                "GetRoomInfo",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.get_room_info(ctx, req)
                                .await?
                                .encode::<
                                    crate::gen_rust::slotpb::v1::GetRoomInfoResp,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(SLOT_SERVICE_GET_ROOM_INFO_SPEC)
    }
}
/// Monomorphic dispatcher for `SlotService`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = SlotServiceServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct SlotServiceServer<T> {
    inner: ::std::sync::Arc<T>,
}
impl<T: SlotService> SlotServiceServer<T> {
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
impl<T> Clone for SlotServiceServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(&self.inner),
        }
    }
}
impl<T: SlotService> ::connectrpc::Dispatcher for SlotServiceServer<T> {
    #[inline]
    fn lookup(
        &self,
        path: &str,
    ) -> Option<::connectrpc::dispatcher::codegen::MethodDescriptor> {
        let method = path.strip_prefix("slotpb.v1.SlotService/")?;
        match method {
            "Play" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(SLOT_SERVICE_PLAY_SPEC),
                )
            }
            "BuyFg" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(SLOT_SERVICE_BUY_FG_SPEC),
                )
            }
            "GetRoomInfo" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(SLOT_SERVICE_GET_ROOM_INFO_SPEC),
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
        let Some(method) = path.strip_prefix("slotpb.v1.SlotService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "Play" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::slotpb::v1::__buffa::view::PlayReqView,
                    >(request.encoded()?, format)?;
                    svc.play(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::slotpb::v1::PlayResp>(format)
                })
            }
            "BuyFg" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::slotpb::v1::__buffa::view::BuyFgReqView,
                    >(request.encoded()?, format)?;
                    svc.buy_fg(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::slotpb::v1::BuyFgResp>(format)
                })
            }
            "GetRoomInfo" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoReqView,
                    >(request.encoded()?, format)?;
                    svc.get_room_info(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::slotpb::v1::GetRoomInfoResp>(format)
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
        let Some(method) = path.strip_prefix("slotpb.v1.SlotService/") else {
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
        let Some(method) = path.strip_prefix("slotpb.v1.SlotService/") else {
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
        let Some(method) = path.strip_prefix("slotpb.v1.SlotService/") else {
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
/// let client = SlotServiceClient::new(conn, config);
/// let response = client.play(request).await?;
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
/// let client = SlotServiceClient::new(http, config);
/// let response = client.play(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// The `OwnedView` derefs to the view, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.play(request).await?.into_view();
/// let name: &str = resp.name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.play(request).await?.into_owned();
/// ```
#[derive(Clone)]
pub struct SlotServiceClient<T> {
    transport: T,
    config: ::connectrpc::client::ClientConfig,
}
impl<T> SlotServiceClient<T>
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
    /// Call the Play RPC. Sends a request to /slotpb.v1.SlotService/Play.
    pub async fn play(
        &self,
        request: crate::gen_rust::slotpb::v1::PlayReq,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::PlayRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.play_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the Play RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn play_with_options(
        &self,
        request: crate::gen_rust::slotpb::v1::PlayReq,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::PlayRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                SLOT_SERVICE_SERVICE_NAME,
                "Play",
                request,
                options,
            )
            .await
    }
    /// Call the BuyFg RPC. Sends a request to /slotpb.v1.SlotService/BuyFg.
    pub async fn buy_fg(
        &self,
        request: crate::gen_rust::slotpb::v1::BuyFgReq,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::BuyFgRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.buy_fg_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the BuyFg RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn buy_fg_with_options(
        &self,
        request: crate::gen_rust::slotpb::v1::BuyFgReq,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::BuyFgRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                SLOT_SERVICE_SERVICE_NAME,
                "BuyFg",
                request,
                options,
            )
            .await
    }
    /// Call the GetRoomInfo RPC. Sends a request to /slotpb.v1.SlotService/GetRoomInfo.
    pub async fn get_room_info(
        &self,
        request: crate::gen_rust::slotpb::v1::GetRoomInfoReq,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.get_room_info_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the GetRoomInfo RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn get_room_info_with_options(
        &self,
        request: crate::gen_rust::slotpb::v1::GetRoomInfoReq,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::slotpb::v1::__buffa::view::GetRoomInfoRespView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                SLOT_SERVICE_SERVICE_NAME,
                "GetRoomInfo",
                request,
                options,
            )
            .await
    }
}
