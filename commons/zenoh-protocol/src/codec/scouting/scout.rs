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
use crate::codec::{RCodec, WCodec, Zenoh060, Zenoh060RCodec};
use crate::message::scouting::Scout;
use crate::message::transport::tmsg;
use crate::message::{imsg, Header};
use zenoh_buffers::traits::{
    reader::{DidntRead, Reader},
    writer::{DidntWrite, Writer},
};
use zenoh_protocol_core::whatami::WhatAmIMatcher;
use zenoh_protocol_core::ZInt;

impl Header for Scout {
    #[inline(always)]
    fn header(&self) -> u8 {
        let mut header = tmsg::id::SCOUT;
        if self.zid_request {
            header |= tmsg::flag::I;
        }
        if self.what.is_some() {
            header |= tmsg::flag::W;
        }
        header
    }
}

impl<W> WCodec<&mut W, &Scout> for Zenoh060
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    fn write(self, writer: &mut W, x: &Scout) -> Self::Output {
        self.write(&mut *writer, x.header())?;
        match x.what {
            Some(w) => {
                let w: ZInt = w.into();
                self.write(&mut *writer, w)
            }
            None => Ok(()),
        }
    }
}

impl<R> RCodec<&mut R, Scout> for Zenoh060
where
    R: Reader,
{
    type Error = DidntRead;

    fn read(self, reader: &mut R) -> Result<Scout, Self::Error> {
        let codec = Zenoh060RCodec {
            header: self.read(&mut *reader)?,
            ..Default::default()
        };
        codec.read(reader)
    }
}

impl<R> RCodec<&mut R, Scout> for Zenoh060RCodec
where
    R: Reader,
{
    type Error = DidntRead;

    fn read(self, reader: &mut R) -> Result<Scout, Self::Error> {
        if imsg::mid(self.header) != imsg::id::SCOUT {
            return Err(DidntRead);
        }

        let zid_request = imsg::has_flag(self.header, tmsg::flag::I);
        let what = if imsg::has_flag(self.header, tmsg::flag::W) {
            let wai: ZInt = self.codec.read(reader)?;
            WhatAmIMatcher::try_from(wai)
        } else {
            None
        };
        Ok(Scout { what, zid_request })
    }
}
