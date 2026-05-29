///Shorthand for `OwnedView<SubscribeRequestView<'static>>`.
pub type OwnedSubscribeRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::SubscribeRequestView<'static>,
>;
///Shorthand for `OwnedView<FrameMessageView<'static>>`.
pub type OwnedFrameMessageView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::FrameMessageView<'static>,
>;
///Shorthand for `OwnedView<JoinRequestView<'static>>`.
pub type OwnedJoinRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::JoinRequestView<'static>,
>;
///Shorthand for `OwnedView<JoinResponseView<'static>>`.
pub type OwnedJoinResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::JoinResponseView<'static>,
>;
///Shorthand for `OwnedView<ShootRequestView<'static>>`.
pub type OwnedShootRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::ShootRequestView<'static>,
>;
///Shorthand for `OwnedView<ShootResponseView<'static>>`.
pub type OwnedShootResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::ShootResponseView<'static>,
>;
///Shorthand for `OwnedView<HitRequestView<'static>>`.
pub type OwnedHitRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::HitRequestView<'static>,
>;
///Shorthand for `OwnedView<HitResponseView<'static>>`.
pub type OwnedHitResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::HitResponseView<'static>,
>;
///Shorthand for `OwnedView<LeaveRequestView<'static>>`.
pub type OwnedLeaveRequestView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::LeaveRequestView<'static>,
>;
///Shorthand for `OwnedView<LeaveResponseView<'static>>`.
pub type OwnedLeaveResponseView = ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::LeaveResponseView<'static>,
>;
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::FrameMessage>
for crate::gen_rust::fishpb::__buffa::view::FrameMessageView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::FrameMessage>
for ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::FrameMessageView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::JoinResponse>
for crate::gen_rust::fishpb::__buffa::view::JoinResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::JoinResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::JoinResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::ShootResponse>
for crate::gen_rust::fishpb::__buffa::view::ShootResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::ShootResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::ShootResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::HitResponse>
for crate::gen_rust::fishpb::__buffa::view::HitResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::HitResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::HitResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::LeaveResponse>
for crate::gen_rust::fishpb::__buffa::view::LeaveResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::gen_rust::fishpb::LeaveResponse>
for ::buffa::view::OwnedView<
    crate::gen_rust::fishpb::__buffa::view::LeaveResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(&**self, codec)
    }
}
/// Full service name for this service.
pub const FISH_SERVICE_SERVICE_NAME: &str = "fishpb.FishService";
/// Static [`Spec`](::connectrpc::Spec) for the server-side `SubscribeFrames` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const FISH_SERVICE_SUBSCRIBE_FRAMES_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/fishpb.FishService/SubscribeFrames",
        ::connectrpc::StreamType::ServerStream,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `JoinRoom` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const FISH_SERVICE_JOIN_ROOM_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/fishpb.FishService/JoinRoom",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `Shoot` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const FISH_SERVICE_SHOOT_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/fishpb.FishService/Shoot",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `Hit` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const FISH_SERVICE_HIT_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/fishpb.FishService/Hit",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the server-side `LeaveRoom` RPC.
///
/// The dispatcher surfaces this on
/// [`RequestContext::spec`](::connectrpc::RequestContext::spec).
pub const FISH_SERVICE_LEAVE_ROOM_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/fishpb.FishService/LeaveRoom",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Service running on the Rust compute pod, serving the Bastion Go gateway
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
pub trait FishService: Send + Sync + 'static {
    /// Streams real-time room frames (ticks) to Bastion
    fn subscribe_frames(
        &self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedSubscribeRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            ::connectrpc::ServiceStream<
                impl ::connectrpc::Encodable<
                    crate::gen_rust::fishpb::FrameMessage,
                > + Send + use<Self>,
            >,
        >,
    > + Send;
    /// Player Actions (Unary gRPC calls)
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn join_room<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedJoinRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::fishpb::JoinResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the Shoot RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn shoot<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedShootRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::fishpb::ShootResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the Hit RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn hit<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedHitRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::fishpb::HitResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the LeaveRoom RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    fn leave_room<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: OwnedLeaveRequestView,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::gen_rust::fishpb::LeaveResponse,
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
pub trait FishServiceExt: FishService {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router;
}
impl<S: FishService> FishServiceExt for S {
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router {
        router
            .route_view_server_stream::<
                _,
                _,
                crate::gen_rust::fishpb::FrameMessage,
            >(
                FISH_SERVICE_SERVICE_NAME,
                "SubscribeFrames",
                ::connectrpc::view_streaming_handler_fn({
                    let svc = ::std::sync::Arc::clone(&self);
                    move |ctx, req| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move { svc.subscribe_frames(ctx, req).await }
                    }
                }),
            )
            .with_spec(FISH_SERVICE_SUBSCRIBE_FRAMES_SPEC)
            .route_view(
                FISH_SERVICE_SERVICE_NAME,
                "JoinRoom",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.join_room(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::fishpb::JoinResponse>(format)
                        }
                    })
                },
            )
            .with_spec(FISH_SERVICE_JOIN_ROOM_SPEC)
            .route_view(
                FISH_SERVICE_SERVICE_NAME,
                "Shoot",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.shoot(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::fishpb::ShootResponse>(format)
                        }
                    })
                },
            )
            .with_spec(FISH_SERVICE_SHOOT_SPEC)
            .route_view(
                FISH_SERVICE_SERVICE_NAME,
                "Hit",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.hit(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::fishpb::HitResponse>(format)
                        }
                    })
                },
            )
            .with_spec(FISH_SERVICE_HIT_SPEC)
            .route_view(
                FISH_SERVICE_SERVICE_NAME,
                "LeaveRoom",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |ctx, req, format| {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            svc.leave_room(ctx, req)
                                .await?
                                .encode::<crate::gen_rust::fishpb::LeaveResponse>(format)
                        }
                    })
                },
            )
            .with_spec(FISH_SERVICE_LEAVE_ROOM_SPEC)
    }
}
/// Monomorphic dispatcher for `FishService`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = FishServiceServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct FishServiceServer<T> {
    inner: ::std::sync::Arc<T>,
}
impl<T: FishService> FishServiceServer<T> {
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
impl<T> Clone for FishServiceServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(&self.inner),
        }
    }
}
impl<T: FishService> ::connectrpc::Dispatcher for FishServiceServer<T> {
    #[inline]
    fn lookup(
        &self,
        path: &str,
    ) -> Option<::connectrpc::dispatcher::codegen::MethodDescriptor> {
        let method = path.strip_prefix("fishpb.FishService/")?;
        match method {
            "SubscribeFrames" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::server_streaming()
                        .with_spec(FISH_SERVICE_SUBSCRIBE_FRAMES_SPEC),
                )
            }
            "JoinRoom" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(FISH_SERVICE_JOIN_ROOM_SPEC),
                )
            }
            "Shoot" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(FISH_SERVICE_SHOOT_SPEC),
                )
            }
            "Hit" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(FISH_SERVICE_HIT_SPEC),
                )
            }
            "LeaveRoom" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(FISH_SERVICE_LEAVE_ROOM_SPEC),
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
        let Some(method) = path.strip_prefix("fishpb.FishService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "JoinRoom" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::fishpb::__buffa::view::JoinRequestView,
                    >(request.encoded()?, format)?;
                    svc.join_room(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::fishpb::JoinResponse>(format)
                })
            }
            "Shoot" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::fishpb::__buffa::view::ShootRequestView,
                    >(request.encoded()?, format)?;
                    svc.shoot(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::fishpb::ShootResponse>(format)
                })
            }
            "Hit" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::fishpb::__buffa::view::HitRequestView,
                    >(request.encoded()?, format)?;
                    svc.hit(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::fishpb::HitResponse>(format)
                })
            }
            "LeaveRoom" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::fishpb::__buffa::view::LeaveRequestView,
                    >(request.encoded()?, format)?;
                    svc.leave_room(ctx, req)
                        .await?
                        .encode::<crate::gen_rust::fishpb::LeaveResponse>(format)
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
        let Some(method) = path.strip_prefix("fishpb.FishService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "SubscribeFrames" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = ::connectrpc::dispatcher::codegen::decode_request_view::<
                        crate::gen_rust::fishpb::__buffa::view::SubscribeRequestView,
                    >(request, format)?;
                    let resp = svc.subscribe_frames(ctx, req).await?;
                    Ok(
                        resp
                            .map_body(|s| ::connectrpc::dispatcher::codegen::encode_response_stream::<
                                crate::gen_rust::fishpb::FrameMessage,
                                _,
                                _,
                            >(s, format)),
                    )
                })
            }
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
        let Some(method) = path.strip_prefix("fishpb.FishService/") else {
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
        let Some(method) = path.strip_prefix("fishpb.FishService/") else {
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
/// let client = FishServiceClient::new(conn, config);
/// let response = client.subscribe_frames(request).await?;
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
/// let client = FishServiceClient::new(http, config);
/// let response = client.subscribe_frames(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// The `OwnedView` derefs to the view, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.subscribe_frames(request).await?.into_view();
/// let name: &str = resp.name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.subscribe_frames(request).await?.into_owned();
/// ```
#[derive(Clone)]
pub struct FishServiceClient<T> {
    transport: T,
    config: ::connectrpc::client::ClientConfig,
}
impl<T> FishServiceClient<T>
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
    /// Call the SubscribeFrames RPC. Sends a request to /fishpb.FishService/SubscribeFrames.
    pub async fn subscribe_frames(
        &self,
        request: crate::gen_rust::fishpb::SubscribeRequest,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::gen_rust::fishpb::__buffa::view::FrameMessageView<'static>,
        >,
        ::connectrpc::ConnectError,
    > {
        self.subscribe_frames_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the SubscribeFrames RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn subscribe_frames_with_options(
        &self,
        request: crate::gen_rust::fishpb::SubscribeRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::gen_rust::fishpb::__buffa::view::FrameMessageView<'static>,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_server_stream(
                &self.transport,
                &self.config,
                FISH_SERVICE_SERVICE_NAME,
                "SubscribeFrames",
                request,
                options,
            )
            .await
    }
    /// Call the JoinRoom RPC. Sends a request to /fishpb.FishService/JoinRoom.
    pub async fn join_room(
        &self,
        request: crate::gen_rust::fishpb::JoinRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::JoinResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.join_room_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the JoinRoom RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn join_room_with_options(
        &self,
        request: crate::gen_rust::fishpb::JoinRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::JoinResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                FISH_SERVICE_SERVICE_NAME,
                "JoinRoom",
                request,
                options,
            )
            .await
    }
    /// Call the Shoot RPC. Sends a request to /fishpb.FishService/Shoot.
    pub async fn shoot(
        &self,
        request: crate::gen_rust::fishpb::ShootRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::ShootResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.shoot_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the Shoot RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn shoot_with_options(
        &self,
        request: crate::gen_rust::fishpb::ShootRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::ShootResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                FISH_SERVICE_SERVICE_NAME,
                "Shoot",
                request,
                options,
            )
            .await
    }
    /// Call the Hit RPC. Sends a request to /fishpb.FishService/Hit.
    pub async fn hit(
        &self,
        request: crate::gen_rust::fishpb::HitRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::HitResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.hit_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the Hit RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn hit_with_options(
        &self,
        request: crate::gen_rust::fishpb::HitRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::HitResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                FISH_SERVICE_SERVICE_NAME,
                "Hit",
                request,
                options,
            )
            .await
    }
    /// Call the LeaveRoom RPC. Sends a request to /fishpb.FishService/LeaveRoom.
    pub async fn leave_room(
        &self,
        request: crate::gen_rust::fishpb::LeaveRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::LeaveResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.leave_room_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the LeaveRoom RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn leave_room_with_options(
        &self,
        request: crate::gen_rust::fishpb::LeaveRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::gen_rust::fishpb::__buffa::view::LeaveResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                FISH_SERVICE_SERVICE_NAME,
                "LeaveRoom",
                request,
                options,
            )
            .await
    }
}
