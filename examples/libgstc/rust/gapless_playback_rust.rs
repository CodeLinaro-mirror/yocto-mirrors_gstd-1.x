/*
 * This file is part of GStreamer Daemon
 * Copyright 2015-2022 Ridgerun, LLC (http://www.ridgerun.com)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

use gstc_rust::{Client, Status};
use std::env;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), Status> {
    let video = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Please provide a video to play");
            return Err(Status::NULL_ARGUMENT);
        }
    };

    let abs_path = PathBuf::from(&video)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(video));
    let uri = format!("file://{}", abs_path.display());

    let client = Client::new("127.0.0.1", 5000, -1, true)?;

    client.pipeline_create("pipe", &format!("playbin uri={}", uri))?;
    println!("Pipeline created successfully!");

    client.pipeline_play("pipe")?;
    println!("Pipeline set to playing!");

    println!("Press enter to stop the pipeline...");
    let (tx, rx) = mpsc::channel::<()>();
    thread::spawn(move || {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        let _ = tx.send(());
    });

    while rx.try_recv().is_err() {
        let message = client.pipeline_bus_wait("pipe", "eos", -1)?;
        if message.status != Status::OK {
            eprintln!("Unable to read from bus: {}", message.status.0);
            break;
        }

        println!("EOS message received!");

        client.pipeline_seek("pipe", 1.0, 3, 1, 1, 0, 1, -1)?;
        println!("Pipeline reset!");
    }

    client.pipeline_stop("pipe")?;
    println!("Pipeline set to null!");

    client.pipeline_delete("pipe")?;
    println!("Pipeline deleted!");

    Ok(())
}
