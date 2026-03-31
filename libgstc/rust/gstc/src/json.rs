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

use crate::Status;

pub(crate) fn json_get_int(json: &str, name: &str) -> Result<i32, Status> {
    let start = find_key_value_start(json, name).ok_or(Status::NotFound)?;
    parse_json_int(json, start)
}

pub(crate) fn json_is_null_field(json: &str, name: &str) -> Result<bool, Status> {
    let start = find_key_value_start(json, name).ok_or(Status::NotFound)?;
    let start = skip_ws(json, start);
    Ok(json[start..].starts_with("null"))
}

pub(crate) fn json_child_string(
    json: &str,
    parent_name: &str,
    data_name: &str,
) -> Result<String, Status> {
    let parent = extract_object_for_key(json, parent_name)?;
    extract_string_for_key(parent, data_name)
}

pub(crate) fn json_child_char_array(
    json: &str,
    parent_name: &str,
    array_name: &str,
    element_name: &str,
) -> Result<Vec<String>, Status> {
    let parent = extract_object_for_key(json, parent_name)?;
    let array = extract_array_for_key(parent, array_name)?;

    let mut out = Vec::new();
    let mut cursor = 1usize;
    while cursor < array.len() {
        let cursor_ws = skip_ws(array, cursor);
        if cursor_ws >= array.len() || array.as_bytes()[cursor_ws] == b']' {
            break;
        }

        if array.as_bytes()[cursor_ws] != b'{' {
            return Err(Status::TypeError);
        }

        let (obj, next_idx) = extract_balanced(array, cursor_ws, b'{', b'}')?;
        out.push(extract_string_for_key(obj, element_name)?);
        cursor = skip_past_comma(array, next_idx);
    }

    Ok(out)
}

fn skip_ws(s: &str, mut idx: usize) -> usize {
    while idx < s.len() && s.as_bytes()[idx].is_ascii_whitespace() {
        idx += 1;
    }
    idx
}

fn skip_past_comma(s: &str, idx: usize) -> usize {
    let idx = skip_ws(s, idx);
    if idx < s.len() && s.as_bytes()[idx] == b',' {
        idx + 1
    } else {
        idx
    }
}

fn find_key_value_start(json: &str, key: &str) -> Option<usize> {
    let pattern = format!("\"{}\"", key);
    if let Some(found) = json.find(&pattern) {
        let key_idx = found + pattern.len();
        let colon_idx = json[key_idx..].find(':')?;
        return Some(key_idx + colon_idx + 1);
    }
    None
}

fn parse_json_int(json: &str, start: usize) -> Result<i32, Status> {
    let start = skip_ws(json, start);
    if start >= json.len() {
        return Err(Status::Malformed);
    }

    let bytes = json.as_bytes();
    let mut end = start;
    if bytes[end] == b'-' {
        end += 1;
    }
    while end < json.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }

    if end == start || (end == start + 1 && bytes[start] == b'-') {
        return Err(Status::TypeError);
    }

    json[start..end]
        .parse::<i32>()
        .map_err(|_| Status::TypeError)
}

fn extract_balanced(
    json: &str,
    start: usize,
    open: u8,
    close: u8,
) -> Result<(&str, usize), Status> {
    let bytes = json.as_bytes();
    if start >= bytes.len() || bytes[start] != open {
        return Err(Status::TypeError);
    }

    let mut depth = 0i32;
    let mut i = start;
    let mut in_string = false;
    let mut escaped = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
        } else if b == b'"' {
            in_string = true;
        } else if b == open {
            depth += 1;
        } else if b == close {
            depth -= 1;
            if depth == 0 {
                return Ok((&json[start..=i], i + 1));
            }
        }
        i += 1;
    }

    Err(Status::Malformed)
}

fn extract_object_for_key<'a>(json: &'a str, key: &str) -> Result<&'a str, Status> {
    let start = find_key_value_start(json, key).ok_or(Status::NotFound)?;
    let start = skip_ws(json, start);
    let (obj, _) = extract_balanced(json, start, b'{', b'}')?;
    Ok(obj)
}

fn extract_array_for_key<'a>(json: &'a str, key: &str) -> Result<&'a str, Status> {
    let start = find_key_value_start(json, key).ok_or(Status::TypeError)?;
    let start = skip_ws(json, start);
    let (arr, _) = extract_balanced(json, start, b'[', b']')?;
    Ok(arr)
}

fn extract_string_for_key(json: &str, key: &str) -> Result<String, Status> {
    let start = find_key_value_start(json, key).ok_or(Status::NotFound)?;
    let start = skip_ws(json, start);
    if start >= json.len() || json.as_bytes()[start] != b'"' {
        return Err(Status::TypeError);
    }

    let mut i = start + 1;
    let mut escaped = false;
    while i < json.len() {
        let b = json.as_bytes()[i];
        if escaped {
            escaped = false;
        } else if b == b'\\' {
            escaped = true;
        } else if b == b'"' {
            return Ok(json[start + 1..i].to_string());
        }
        i += 1;
    }

    Err(Status::Malformed)
}
