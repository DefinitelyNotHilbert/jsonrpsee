// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! # jsonrpsee-ws-server
//!
//! `jsonrpsee-ws-server` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket server library that's is built for `async/await`.

extern crate alloc;

mod future;
mod server;

#[cfg(test)]
mod tests;

pub use future::{ShutdownWaiter as WsShutdownWaiter, StopHandle as WsStopHandle};
pub use jsonrpsee_types as types;
pub use jsonrpsee_utils::server::rpc_module::{RpcModule, SubscriptionSink};
pub use server::{Builder as WsServerBuilder, Server as WsServer};
