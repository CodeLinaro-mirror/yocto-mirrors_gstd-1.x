/*
 * This file is part of GStreamer Daemon
 * Copyright 2015-2026 RidgeRun, LLC (http://www.ridgerun.com)
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * 1. Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 * notice, this list of conditions and the following disclaimer in the
 * documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status(pub i32);

impl Status {
    pub const OK: Status = Status(0);
    pub const NULL_ARGUMENT: Status = Status(-1);
    pub const UNREACHABLE: Status = Status(-2);
    pub const TIMEOUT: Status = Status(-3);
    pub const OOM: Status = Status(-4);
    pub const TYPE_ERROR: Status = Status(-5);
    pub const MALFORMED: Status = Status(-6);
    pub const NOT_FOUND: Status = Status(-7);
    pub const SEND_ERROR: Status = Status(-8);
    pub const RECV_ERROR: Status = Status(-9);
    pub const SOCKET_ERROR: Status = Status(-10);
    pub const THREAD_ERROR: Status = Status(-11);
    pub const BUS_TIMEOUT: Status = Status(-12);
    pub const SOCKET_TIMEOUT: Status = Status(-13);
    pub const LONG_RESPONSE: Status = Status(-14);

    pub fn is_ok(self) -> bool {
        self == Status::OK
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Status::OK => "GSTC_OK",
            Status::NULL_ARGUMENT => "GSTC_NULL_ARGUMENT",
            Status::UNREACHABLE => "GSTC_UNREACHABLE",
            Status::TIMEOUT => "GSTC_TIMEOUT",
            Status::OOM => "GSTC_OOM",
            Status::TYPE_ERROR => "GSTC_TYPE_ERROR",
            Status::MALFORMED => "GSTC_MALFORMED",
            Status::NOT_FOUND => "GSTC_NOT_FOUND",
            Status::SEND_ERROR => "GSTC_SEND_ERROR",
            Status::RECV_ERROR => "GSTC_RECV_ERROR",
            Status::SOCKET_ERROR => "GSTC_SOCKET_ERROR",
            Status::THREAD_ERROR => "GSTC_THREAD_ERROR",
            Status::BUS_TIMEOUT => "GSTC_BUS_TIMEOUT",
            Status::SOCKET_TIMEOUT => "GSTC_SOCKET_TIMEOUT",
            Status::LONG_RESPONSE => "GSTC_LONG_RESPONSE",
            _ => "GSTC_UNKNOWN",
        };

        write!(f, "{} ({})", name, self.0)
    }
}

impl std::error::Error for Status {}
