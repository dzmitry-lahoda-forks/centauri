// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// TODO: disable unwraps:
// https://github.com/informalsystems/ibc-rs/issues/987
// #![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]
#![deny(
    // warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(test), deny(rust_2018_idioms))]
#![forbid(unsafe_code)]

//! This library implements the InterBlockchain Communication (IBC) protocol in Rust. IBC is
//! a distributed protocol that enables communication between distinct sovereign blockchains.
//! Loose analogies may be drawn between the IBC protocol and the TCP/UDP protocols that enable
//! communication over the internet via packet streaming. Indeed, IBC also encodes the notion of
//! ordered and unordered packet streams.
//!
//! The layout of this crate mirrors the classification of the [Interchain
//! Standards][ics-standards]. The classification consists of [Core][core], [Clients][clients],
//! [Applications][applications], and [Relayer][relayer].
//!
//! `Core` consists of the designs and logic pertaining to the transport, authentication, and
//! ordering layers of the IBC protocol, the fundamental pieces.
//!
//! `Clients` consists of implementations of client verification algorithms (following the base
//! client interface that is defined in `Core`) for specific types of chains. A chain uses these
//! verification algorithms to verify the state of remote chains.
//!
//! `Applications` consists of various packet encoding and processing semantics which underpin the
//! various types of transactions that users can perform on any IBC-compliant chain.
//!
//! `Relayer` contains utilities for testing the `ibc` crate against the [Hermes IBC
//! relayer][relayer-repo]. It acts as scaffolding for gluing the `ibc` crate with Hermes for
//! testing purposes.
//!
//! [core]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/core
//! [clients]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/clients
//! [applications]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/applications
//! [ics-standards]: https://github.com/cosmos/ibc#interchain-standards
//! [relayer]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/relayer
//! [relayer-repo]: https://github.com/informalsystems/ibc-rs/tree/master/relayer

extern crate alloc;
#[allow(unused_imports)]
#[macro_use]
extern crate derive;
#[cfg(any(test, feature = "mocks"))]
#[macro_use]
extern crate serde;
#[cfg(feature = "std")]
extern crate std;

pub mod prelude;

pub mod applications;
pub mod bigint;
pub mod core;
pub mod events;
pub mod handler;
pub mod keys;
#[macro_use]
pub mod macros;
pub mod proofs;
mod serializers;
pub mod signer;
pub mod timestamp;
pub mod tx_msg; // Context mock, the underlying host chain, and client types: for testing all handlers.

pub mod protobuf {
	pub use tendermint_proto::*;
}

/// Re-export of ICS 002 Height domain type
pub type Height = core::ics02_client::height::Height;

#[cfg(any(test, feature = "mocks"))]
pub mod test;

#[cfg(any(test, feature = "mocks"))]
pub mod test_utils;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;
