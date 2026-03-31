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
pub enum Status {
    OK,
    NULL_ARGUMENT,
    UNREACHABLE,
    TIMEOUT,
    OOM,
    TYPE_ERROR,
    MALFORMED,
    NOT_FOUND,
    SEND_ERROR,
    RECV_ERROR,
    SOCKET_ERROR,
    THREAD_ERROR,
    BUS_TIMEOUT,
    SOCKET_TIMEOUT,
    LONG_RESPONSE,
    UNKNOWN(i32),
}

impl Status {
    pub fn from_code(code: i32) -> Status {
        match code {
            0 => Status::OK,
            -1 => Status::NULL_ARGUMENT,
            -2 => Status::UNREACHABLE,
            -3 => Status::TIMEOUT,
            -4 => Status::OOM,
            -5 => Status::TYPE_ERROR,
            -6 => Status::MALFORMED,
            -7 => Status::NOT_FOUND,
            -8 => Status::SEND_ERROR,
            -9 => Status::RECV_ERROR,
            -10 => Status::SOCKET_ERROR,
            -11 => Status::THREAD_ERROR,
            -12 => Status::BUS_TIMEOUT,
            -13 => Status::SOCKET_TIMEOUT,
            -14 => Status::LONG_RESPONSE,
            other => Status::UNKNOWN(other),
        }
    }

    pub fn code(self) -> i32 {
        match self {
            Status::OK => 0,
            Status::NULL_ARGUMENT => -1,
            Status::UNREACHABLE => -2,
            Status::TIMEOUT => -3,
            Status::OOM => -4,
            Status::TYPE_ERROR => -5,
            Status::MALFORMED => -6,
            Status::NOT_FOUND => -7,
            Status::SEND_ERROR => -8,
            Status::RECV_ERROR => -9,
            Status::SOCKET_ERROR => -10,
            Status::THREAD_ERROR => -11,
            Status::BUS_TIMEOUT => -12,
            Status::SOCKET_TIMEOUT => -13,
            Status::LONG_RESPONSE => -14,
            Status::UNKNOWN(code) => code,
        }
    }

    pub fn is_ok(self) -> bool {
        matches!(self, Status::OK)
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
            Status::UNKNOWN(_) => "GSTC_UNKNOWN",
        };

        write!(f, "{} ({})", name, self.code())
    }
}

impl std::error::Error for Status {}
