//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
mod init;
use crate::message::core::Attachment;

// Zenoh messages at zenoh-transport level
#[derive(Debug, Clone, PartialEq)]
pub enum TransportBody {
    // Scout(Scout),
    // Hello(Hello),
    // InitSyn(InitSyn),
    // InitAck(InitAck),
    // OpenSyn(OpenSyn),
    // OpenAck(OpenAck),
    // Join(Join),
    // Close(Close),
    // Sync(Sync),
    // AckNack(AckNack),
    // KeepAlive(KeepAlive),
    // Ping(Ping),
    // Pong(Pong),
    // Frame(Frame),
}

#[derive(Debug, Clone)]
pub struct TransportMessage {
    pub body: TransportBody,
    pub attachment: Option<Attachment>,
    #[cfg(feature = "stats")]
    pub size: Option<std::num::NonZeroUsize>,
}
