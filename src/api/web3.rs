//! `Web3` namespace

use crate::{
    api::Namespace,
    helpers::{self, CallFuture},
    transports::ic_http_client::CallOptions,
    types::{Bytes, H256},
    Transport,
};

/// `Web3` namespace
#[derive(Debug, Clone)]
pub struct Web3<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Web3<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Web3 { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Web3<T> {
    /// Returns client version
    pub fn client_version(&self, options: CallOptions) -> CallFuture<String, T::Out> {
        CallFuture::new(self.transport.execute("web3_clientVersion", vec![], options))
    }

    /// Returns sha3 of the given data
    pub fn sha3(&self, bytes: Bytes, options: CallOptions) -> CallFuture<H256, T::Out> {
        let bytes = helpers::serialize(&bytes);
        CallFuture::new(self.transport.execute("web3_sha3", vec![bytes], options))
    }
}

#[cfg(test)]
mod tests {
    use super::Web3;
    use crate::{api::Namespace, rpc::Value, transports::ic_http_client::CallOptions, types::H256};
    use hex_literal::hex;

    rpc_test! (
      Web3:client_version, CallOptions::default() => "web3_clientVersion", Vec::<String>::new();
      Value::String("Test123".into()) => "Test123"
    );

    rpc_test! (
      Web3:sha3, hex!("01020304"),CallOptions::default()
      =>
      "web3_sha3", vec![r#""0x01020304""#];
      Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => H256::from_low_u64_be(0x123)
    );
}
