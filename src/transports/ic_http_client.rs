//! IC http client

use candid::CandidType;
use candid::{candid_method, Principal};
use derive_builder::Builder;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformContext, TransformFunc,
};
use jsonrpc_core::Request;
use serde::{self, Deserialize, Serialize};

// #[derive(CandidType, Deserialize, Debug)]
// pub struct CanisterHttpRequestArgs {
//     pub url: String,
//     pub max_response_bytes: Option<u64>,
//     pub headers: Vec<HttpHeader>,
//     pub body: Option<Vec<u8>>,
//     pub http_method: HttpMethod,
//     pub transform_method_name: Option<String>,
// }

const HTTP_OUTCALL_PRICE: u128 = 400_000_000;
const COST_PER_BYTE: u128 = 100_000;
const BYTES: u128 = 3_355_443_200_000;

#[derive(Clone, Debug)]
pub struct ICHttpClient {
    pub max_response_bytes: u64,
}

#[derive(Builder, Default, Clone, Debug, PartialEq, Eq)]
pub struct CallOptions {
    max_resp: Option<u64>,
    cycles: Option<u64>,
    transform: Option<TransformContext>,
}

impl ICHttpClient {
    pub fn new(max_resp: Option<u64>) -> Self {
        ICHttpClient {
            max_response_bytes: if let Some(v) = max_resp { v } else { 500_000 },
        }
    }

    pub fn set_max_response_bytes(&mut self, v: u64) {
        self.max_response_bytes = v;
    }

    async fn request(
        &self,
        url: String,
        req_type: HttpMethod,
        req_headers: Vec<HttpHeader>,
        payload: &Request,
        options: CallOptions,
    ) -> Result<Vec<u8>, String> {
        let request = CanisterHttpRequestArgument {
            url: url.clone(),
            max_response_bytes: if let Some(v) = options.max_resp {
                Some(v)
            } else {
                Some(self.max_response_bytes)
            },
            method: req_type,
            headers: req_headers,
            body: Some(serde_json::to_vec(&payload).unwrap()),
            // transform: Some(TransformType::Function(TransformFunc(candid::Func {
            //     principal: ic_cdk::api::id(),
            //     method: "transform".to_string(),
            // }))),
            transform: match options.transform {
                Some(t) => Some(t),
                None => Some(TransformContext {
                    function: TransformFunc(candid::Func {
                        principal: ic_cdk::api::id(),
                        method: "transform".to_string(),
                    }),
                    context: vec![],
                }),
            },
        };

        match http_request(request, HTTP_OUTCALL_PRICE + (BYTES * COST_PER_BYTE)).await {
            Ok((result,)) => Ok(result.body),
            Err((r, m)) => {
                let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
                ic_cdk::api::print(message.clone());
                Err(message)
            }
        }
    }

    pub async fn get(&self, url: String, payload: &Request, options: CallOptions) -> Result<Vec<u8>, String> {
        let request_headers = vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }];

        self.request(url, HttpMethod::GET, request_headers, payload, options)
            .await
    }

    pub async fn post(&self, url: String, payload: &Request, options: CallOptions) -> Result<Vec<u8>, String> {
        let request_headers = vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }];

        self.request(url, HttpMethod::POST, request_headers, payload, options)
            .await
    }
}
