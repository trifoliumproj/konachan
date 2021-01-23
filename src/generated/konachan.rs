#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GalleryRequest {
    #[prost(message, repeated, tag = "1")]
    pub tags: ::std::vec::Vec<Tag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GalleryResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::std::vec::Vec<Post>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Post {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub tags: std::string::String,
    #[prost(int32, tag = "3")]
    pub created_at: i32,
    #[prost(int32, tag = "4")]
    pub creator_id: i32,
    #[prost(string, tag = "5")]
    pub author: std::string::String,
    #[prost(int32, tag = "6")]
    pub change: i32,
    #[prost(string, tag = "7")]
    pub source: std::string::String,
    #[prost(int32, tag = "8")]
    pub score: i32,
    #[prost(string, tag = "9")]
    pub md5: std::string::String,
    #[prost(int32, tag = "10")]
    pub file_size: i32,
    #[prost(string, tag = "11")]
    pub file_url: std::string::String,
    #[prost(bool, tag = "12")]
    pub is_shown_in_index: bool,
    #[prost(string, tag = "13")]
    pub preview_url: std::string::String,
    #[prost(int32, tag = "14")]
    pub preview_width: i32,
    #[prost(int32, tag = "15")]
    pub preview_height: i32,
    #[prost(int32, tag = "16")]
    pub actual_preview_width: i32,
    #[prost(int32, tag = "17")]
    pub actual_preview_height: i32,
    #[prost(string, tag = "18")]
    pub sample_url: std::string::String,
    #[prost(int32, tag = "19")]
    pub sample_width: i32,
    #[prost(int32, tag = "20")]
    pub sample_height: i32,
    #[prost(int32, tag = "21")]
    pub sample_file_size: i32,
    #[prost(string, tag = "22")]
    pub jpeg_url: std::string::String,
    #[prost(int32, tag = "23")]
    pub jpeg_width: i32,
    #[prost(int32, tag = "24")]
    pub jpeg_height: i32,
    #[prost(int32, tag = "25")]
    pub jpeg_file_size: i32,
    #[prost(string, tag = "26")]
    pub rating: std::string::String,
    #[prost(bool, tag = "27")]
    pub has_children: bool,
    #[prost(message, optional, tag = "28")]
    pub parent_id: ::std::option::Option<i32>,
    #[prost(string, tag = "29")]
    pub status: std::string::String,
    #[prost(int32, tag = "30")]
    pub width: i32,
    #[prost(int32, tag = "31")]
    pub height: i32,
    /// frames_pending_string = ""
    /// frames_pending = []
    /// frames_string = ""
    /// frames = []
    #[prost(bool, tag = "32")]
    pub is_held: bool,
}
#[doc = r" Generated server implementations."]
pub mod api_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApiServer."]
    #[async_trait]
    pub trait Api: Send + Sync + 'static {
        async fn gallery(
            &self,
            request: tonic::Request<super::GalleryRequest>,
        ) -> Result<tonic::Response<super::GalleryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ApiServer<T: Api> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Api> ApiServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ApiServer<T>
    where
        T: Api,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/konachan.API/Gallery" => {
                    #[allow(non_camel_case_types)]
                    struct GallerySvc<T: Api>(pub Arc<T>);
                    impl<T: Api> tonic::server::UnaryService<super::GalleryRequest> for GallerySvc<T> {
                        type Response = super::GalleryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GalleryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).gallery(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GallerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Api> Clone for ApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Api> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Api> tonic::transport::NamedService for ApiServer<T> {
        const NAME: &'static str = "konachan.API";
    }
}
