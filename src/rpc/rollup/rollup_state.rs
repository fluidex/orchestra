#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct L2BlocksQueryRequest {
    #[prost(int64, tag = "1")]
    pub offset: i64,
    #[prost(int64, tag = "2")]
    pub limit: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct L2BlocksQueryResponse {
    #[prost(int64, tag = "1")]
    pub total: i64,
    #[prost(message, repeated, tag = "2")]
    pub blocks: ::prost::alloc::vec::Vec<l2_blocks_query_response::BlockSummary>,
}
/// Nested message and enum types in `L2BlocksQueryResponse`.
pub mod l2_blocks_query_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct BlockSummary {
        #[prost(int64, tag = "1")]
        pub block_height: i64,
        #[prost(string, tag = "2")]
        pub merkle_root: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub block_time: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct L2BlockQueryRequest {
    #[prost(int64, tag = "1")]
    pub block_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct L2BlockQueryResponse {
    #[prost(uint64, tag = "1")]
    pub tx_num: u64,
    #[prost(uint64, tag = "2")]
    pub real_tx_num: u64,
    #[prost(int64, tag = "3")]
    pub created_time: i64,
    #[prost(enumeration = "BlockStatus", tag = "4")]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub new_root: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub l1_tx_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub txs: ::prost::alloc::vec::Vec<l2_block_query_response::EncodedTx>,
    #[prost(message, repeated, tag = "8")]
    pub decoded_txs: ::prost::alloc::vec::Vec<l2_block_query_response::DecodedTx>,
    #[prost(enumeration = "TxType", repeated, tag = "9")]
    pub txs_type: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `L2BlockQueryResponse`.
pub mod l2_block_query_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct EncodedTx {
        /// TODO: Fixes to decoding TX in issue #132.
        #[prost(string, repeated, tag = "1")]
        pub detail: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DecodedTx {
        #[prost(message, optional, tag = "1")]
        pub deposit_tx: ::core::option::Option<super::DepositTx>,
        #[prost(message, optional, tag = "2")]
        pub withdraw_tx: ::core::option::Option<super::WithdrawTx>,
        #[prost(message, optional, tag = "3")]
        pub transfer_tx: ::core::option::Option<super::TransferTx>,
        #[prost(message, optional, tag = "4")]
        pub spot_trade_tx: ::core::option::Option<super::SpotTradeTx>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TokenBalanceQueryRequest {
    #[prost(uint32, tag = "1")]
    pub account_id: u32,
    #[prost(uint32, optional, tag = "2")]
    pub token_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub token_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub token_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TokenBalanceQueryResponse {
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub balance_raw: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub precision: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DepositTx {
    #[prost(uint32, tag = "1")]
    pub account_id: u32,
    #[prost(uint32, tag = "2")]
    pub token_id: u32,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub new_balance: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WithdrawTx {
    #[prost(uint32, tag = "1")]
    pub account_id: u32,
    #[prost(uint32, tag = "2")]
    pub token_id: u32,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub new_balance: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransferTx {
    #[prost(uint32, tag = "1")]
    pub from: u32,
    #[prost(uint32, tag = "2")]
    pub to: u32,
    #[prost(uint32, tag = "3")]
    pub token_id: u32,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub from_old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub from_new_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub to_old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub to_new_balance: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SpotTradeTx {
    #[prost(uint32, tag = "1")]
    pub order1_account_id: u32,
    #[prost(uint32, tag = "2")]
    pub order2_account_id: u32,
    #[prost(uint32, tag = "3")]
    pub token_id_1to2: u32,
    #[prost(uint32, tag = "4")]
    pub token_id_2to1: u32,
    #[prost(string, tag = "5")]
    pub amount_1to2: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub amount_2to1: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub account1_token_buy_new_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub account1_token_buy_old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub account1_token_sell_new_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub account1_token_sell_old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub account2_token_buy_new_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub account2_token_buy_old_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub account2_token_sell_new_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub account2_token_sell_old_balance: ::prost::alloc::string::String,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum BlockStatus {
    Uncommited = 0,
    Commited = 1,
    Verified = 2,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum TxType {
    Nop = 0,
    Deposit = 1,
    Transfer = 2,
    Withdraw = 3,
    PlaceOrder = 4,
    SpotTrade = 5,
}
#[doc = r" Generated client implementations."]
pub mod rollup_state_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RollupStateClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RollupStateClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RollupStateClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RollupStateClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RollupStateClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn l2_blocks_query(
            &mut self,
            request: impl tonic::IntoRequest<super::L2BlocksQueryRequest>,
        ) -> Result<tonic::Response<super::L2BlocksQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rollup_state.RollupState/L2BlocksQuery");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn l2_block_query(
            &mut self,
            request: impl tonic::IntoRequest<super::L2BlockQueryRequest>,
        ) -> Result<tonic::Response<super::L2BlockQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rollup_state.RollupState/L2BlockQuery");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn token_balance_query(
            &mut self,
            request: impl tonic::IntoRequest<super::TokenBalanceQueryRequest>,
        ) -> Result<tonic::Response<super::TokenBalanceQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rollup_state.RollupState/TokenBalanceQuery");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod rollup_state_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RollupStateServer."]
    #[async_trait]
    pub trait RollupState: Send + Sync + 'static {
        async fn l2_blocks_query(
            &self,
            request: tonic::Request<super::L2BlocksQueryRequest>,
        ) -> Result<tonic::Response<super::L2BlocksQueryResponse>, tonic::Status>;
        async fn l2_block_query(
            &self,
            request: tonic::Request<super::L2BlockQueryRequest>,
        ) -> Result<tonic::Response<super::L2BlockQueryResponse>, tonic::Status>;
        async fn token_balance_query(
            &self,
            request: tonic::Request<super::TokenBalanceQueryRequest>,
        ) -> Result<tonic::Response<super::TokenBalanceQueryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RollupStateServer<T: RollupState> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RollupState> RollupStateServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RollupStateServer<T>
    where
        T: RollupState,
        B: Body + Send + Sync + 'static,
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
                "/rollup_state.RollupState/L2BlocksQuery" => {
                    #[allow(non_camel_case_types)]
                    struct L2BlocksQuerySvc<T: RollupState>(pub Arc<T>);
                    impl<T: RollupState> tonic::server::UnaryService<super::L2BlocksQueryRequest>
                        for L2BlocksQuerySvc<T>
                    {
                        type Response = super::L2BlocksQueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::L2BlocksQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).l2_blocks_query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = L2BlocksQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rollup_state.RollupState/L2BlockQuery" => {
                    #[allow(non_camel_case_types)]
                    struct L2BlockQuerySvc<T: RollupState>(pub Arc<T>);
                    impl<T: RollupState> tonic::server::UnaryService<super::L2BlockQueryRequest>
                        for L2BlockQuerySvc<T>
                    {
                        type Response = super::L2BlockQueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::L2BlockQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).l2_block_query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = L2BlockQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rollup_state.RollupState/TokenBalanceQuery" => {
                    #[allow(non_camel_case_types)]
                    struct TokenBalanceQuerySvc<T: RollupState>(pub Arc<T>);
                    impl<T: RollupState>
                        tonic::server::UnaryService<super::TokenBalanceQueryRequest>
                        for TokenBalanceQuerySvc<T>
                    {
                        type Response = super::TokenBalanceQueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TokenBalanceQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).token_balance_query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TokenBalanceQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RollupState> Clone for RollupStateServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RollupState> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RollupState> tonic::transport::NamedService for RollupStateServer<T> {
        const NAME: &'static str = "rollup_state.RollupState";
    }
}
