/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo run -p api_generator
// -----------------------------------------------
#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Delete Lifecycle API"]
pub enum IlmDeleteLifecycleParts<'b> {
    #[doc = "Policy"]
    Policy(&'b str),
}
impl<'b> IlmDeleteLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Delete Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmDeleteLifecycleParts::Policy(ref policy) => {
                let encoded_policy: Cow<str> =
                    percent_encode(policy.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(encoded_policy.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-delete-lifecycle.html)\n\nDeletes the specified lifecycle policy definition. A currently used policy cannot be deleted."]
pub struct IlmDeleteLifecycle<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IlmDeleteLifecycleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IlmDeleteLifecycle<'a, 'b> {
    #[doc = "Creates a new instance of [IlmDeleteLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmDeleteLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmDeleteLifecycle {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Delete Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Explain Lifecycle API"]
pub enum IlmExplainLifecycleParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IlmExplainLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Explain Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmExplainLifecycleParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ilm/explain");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Explain Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-explain-lifecycle.html)\n\nRetrieves information about the index's current lifecycle state, such as the currently executing phase, action, and step."]
pub struct IlmExplainLifecycle<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IlmExplainLifecycleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    only_errors: Option<bool>,
    only_managed: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IlmExplainLifecycle<'a, 'b> {
    #[doc = "Creates a new instance of [IlmExplainLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmExplainLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmExplainLifecycle {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            only_errors: None,
            only_managed: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "filters the indices included in the response to ones in an ILM error state, implies only_managed"]
    pub fn only_errors(mut self, only_errors: bool) -> Self {
        self.only_errors = Some(only_errors);
        self
    }
    #[doc = "filters the indices included in the response to ones managed by ILM"]
    pub fn only_managed(mut self, only_managed: bool) -> Self {
        self.only_managed = Some(only_managed);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Explain Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "only_errors")]
                only_errors: Option<bool>,
                #[serde(rename = "only_managed")]
                only_managed: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                only_errors: self.only_errors,
                only_managed: self.only_managed,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Get Lifecycle API"]
pub enum IlmGetLifecycleParts<'b> {
    #[doc = "Policy"]
    Policy(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> IlmGetLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Get Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmGetLifecycleParts::Policy(ref policy) => {
                let encoded_policy: Cow<str> =
                    percent_encode(policy.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(encoded_policy.as_ref());
                p.into()
            }
            IlmGetLifecycleParts::None => "/_ilm/policy".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-get-lifecycle.html)\n\nReturns the specified policy definition. Includes the policy version and last modified date."]
pub struct IlmGetLifecycle<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IlmGetLifecycleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IlmGetLifecycle<'a, 'b> {
    #[doc = "Creates a new instance of [IlmGetLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmGetLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmGetLifecycle {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Get Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Get Status API"]
pub enum IlmGetStatusParts {
    #[doc = "No parts"]
    None,
}
impl IlmGetStatusParts {
    #[doc = "Builds a relative URL path to the Ilm Get Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmGetStatusParts::None => "/_ilm/status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-get-status.html)\n\nRetrieves the current index lifecycle management (ILM) status."]
pub struct IlmGetStatus<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IlmGetStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IlmGetStatus<'a, 'b> {
    #[doc = "Creates a new instance of [IlmGetStatus]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        IlmGetStatus {
            client,
            parts: IlmGetStatusParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Get Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Move To Step API"]
pub enum IlmMoveToStepParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IlmMoveToStepParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Move To Step API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmMoveToStepParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/_ilm/move/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Move To Step API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-move-to-step.html)\n\nManually moves an index into the specified step and executes that step."]
pub struct IlmMoveToStep<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmMoveToStepParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmMoveToStep<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmMoveToStep] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmMoveToStepParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmMoveToStep {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmMoveToStep<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmMoveToStep {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Move To Step API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Put Lifecycle API"]
pub enum IlmPutLifecycleParts<'b> {
    #[doc = "Policy"]
    Policy(&'b str),
}
impl<'b> IlmPutLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Put Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmPutLifecycleParts::Policy(ref policy) => {
                let encoded_policy: Cow<str> =
                    percent_encode(policy.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(encoded_policy.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-put-lifecycle.html)\n\nCreates a lifecycle policy"]
pub struct IlmPutLifecycle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmPutLifecycleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmPutLifecycle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmPutLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmPutLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmPutLifecycle {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmPutLifecycle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmPutLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Put Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Remove Policy API"]
pub enum IlmRemovePolicyParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IlmRemovePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Remove Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmRemovePolicyParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ilm/remove");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Remove Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-remove-policy.html)\n\nRemoves the assigned lifecycle policy and stops managing the specified index"]
pub struct IlmRemovePolicy<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmRemovePolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmRemovePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmRemovePolicy] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmRemovePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmRemovePolicy {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmRemovePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmRemovePolicy {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Remove Policy API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Retry API"]
pub enum IlmRetryParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IlmRetryParts<'b> {
    #[doc = "Builds a relative URL path to the Ilm Retry API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmRetryParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ilm/retry");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Retry API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-retry-policy.html)\n\nRetries executing the policy for an index that is in the ERROR step."]
pub struct IlmRetry<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmRetryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmRetry<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmRetry] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IlmRetryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IlmRetry {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmRetry<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmRetry {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Retry API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Start API"]
pub enum IlmStartParts {
    #[doc = "No parts"]
    None,
}
impl IlmStartParts {
    #[doc = "Builds a relative URL path to the Ilm Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmStartParts::None => "/_ilm/start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-start.html)\n\nStart the index lifecycle management (ILM) plugin."]
pub struct IlmStart<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmStartParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmStart]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        IlmStart {
            client,
            parts: IlmStartParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmStart {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Start API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Stop API"]
pub enum IlmStopParts {
    #[doc = "No parts"]
    None,
}
impl IlmStopParts {
    #[doc = "Builds a relative URL path to the Ilm Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmStopParts::None => "/_ilm/stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-stop.html)\n\nHalts all lifecycle management operations and stops the index lifecycle management (ILM) plugin"]
pub struct IlmStop<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IlmStopParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IlmStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmStop]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        IlmStop {
            client,
            parts: IlmStopParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmStop {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ilm Stop API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Index Lifecycle Management APIs"]
pub struct Ilm<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Ilm<'a> {
    #[doc = "Creates a new instance of [Ilm]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Ilm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-delete-lifecycle.html)\n\nDeletes the specified lifecycle policy definition. A currently used policy cannot be deleted."]
    pub fn delete_lifecycle<'b>(
        &'a self,
        parts: IlmDeleteLifecycleParts<'b>,
    ) -> IlmDeleteLifecycle<'a, 'b> {
        IlmDeleteLifecycle::new(&self.client, parts)
    }
    #[doc = "[Ilm Explain Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-explain-lifecycle.html)\n\nRetrieves information about the index's current lifecycle state, such as the currently executing phase, action, and step."]
    pub fn explain_lifecycle<'b>(
        &'a self,
        parts: IlmExplainLifecycleParts<'b>,
    ) -> IlmExplainLifecycle<'a, 'b> {
        IlmExplainLifecycle::new(&self.client, parts)
    }
    #[doc = "[Ilm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-get-lifecycle.html)\n\nReturns the specified policy definition. Includes the policy version and last modified date."]
    pub fn get_lifecycle<'b>(&'a self, parts: IlmGetLifecycleParts<'b>) -> IlmGetLifecycle<'a, 'b> {
        IlmGetLifecycle::new(&self.client, parts)
    }
    #[doc = "[Ilm Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-get-status.html)\n\nRetrieves the current index lifecycle management (ILM) status."]
    pub fn get_status<'b>(&'a self) -> IlmGetStatus<'a, 'b> {
        IlmGetStatus::new(&self.client)
    }
    #[doc = "[Ilm Move To Step API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-move-to-step.html)\n\nManually moves an index into the specified step and executes that step."]
    pub fn move_to_step<'b>(&'a self, parts: IlmMoveToStepParts<'b>) -> IlmMoveToStep<'a, 'b, ()> {
        IlmMoveToStep::new(&self.client, parts)
    }
    #[doc = "[Ilm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-put-lifecycle.html)\n\nCreates a lifecycle policy"]
    pub fn put_lifecycle<'b>(
        &'a self,
        parts: IlmPutLifecycleParts<'b>,
    ) -> IlmPutLifecycle<'a, 'b, ()> {
        IlmPutLifecycle::new(&self.client, parts)
    }
    #[doc = "[Ilm Remove Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-remove-policy.html)\n\nRemoves the assigned lifecycle policy and stops managing the specified index"]
    pub fn remove_policy<'b>(
        &'a self,
        parts: IlmRemovePolicyParts<'b>,
    ) -> IlmRemovePolicy<'a, 'b, ()> {
        IlmRemovePolicy::new(&self.client, parts)
    }
    #[doc = "[Ilm Retry API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-retry-policy.html)\n\nRetries executing the policy for an index that is in the ERROR step."]
    pub fn retry<'b>(&'a self, parts: IlmRetryParts<'b>) -> IlmRetry<'a, 'b, ()> {
        IlmRetry::new(&self.client, parts)
    }
    #[doc = "[Ilm Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-start.html)\n\nStart the index lifecycle management (ILM) plugin."]
    pub fn start<'b>(&'a self) -> IlmStart<'a, 'b, ()> {
        IlmStart::new(&self.client)
    }
    #[doc = "[Ilm Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/ilm-stop.html)\n\nHalts all lifecycle management operations and stops the index lifecycle management (ILM) plugin"]
    pub fn stop<'b>(&'a self) -> IlmStop<'a, 'b, ()> {
        IlmStop::new(&self.client)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Index Lifecycle Management APIs"]
    pub fn ilm(&self) -> Ilm {
        Ilm::new(&self)
    }
}
