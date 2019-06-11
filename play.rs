// Copyright © 2019 Bart Massey

//! Portaudio core dump bug demo.

use portaudio as pa;
use std::error::Error;
const SAMPLE_RATE: f32 = 48000.0;
const OUT_FRAMES: usize = 64;
const TAU: f32 = 2.0 * std::f32::consts::PI;

/// Gather samples and post for playback.
pub fn play(mut samples: Box<Iterator<Item=f32>>) -> Result<(), Box<Error>> {

    // Create and initialize audio output.
    let out = pa::PortAudio::new()?;
    let mut settings = out.default_output_stream_settings(
        1, // 1 channel.
        SAMPLE_RATE as f64,
        0_u32, // Least possible buffer.
    )?;
    settings.flags = pa::stream_flags::CLIP_OFF;
    let mut stream = out.open_blocking_stream(settings)?;

    stream.start()?;

    // Write all the samples present.
    loop {
        // Build a sample buffer.
        let buf: Vec<i16> = (&mut samples)
            .take(OUT_FRAMES)
            .map(|s| f32::floor(s * 32768.0f32) as i16)
            .collect();

        // Write the sample buffer.
        stream.write(buf.len() as u32, |out| {
            assert_eq!(out.len(), buf.len());
            for i in 0..out.len() {
                out[i] = buf[i];
            }
        })?;

        // Handle end condition.
        if buf.len() < OUT_FRAMES {
            break;
        }
    }

    stream.stop()?;
    stream.close()?;

    Ok(())
}

fn main() {
    let mut signal = vec![0.0; 4096];
    for (i, s) in signal.iter_mut().enumerate() {
        *s = 0.5 * f32::sin(1000.0 * TAU * i as f32 / SAMPLE_RATE);
    }
    let samples = Box::new(signal.into_iter());
    play(samples).unwrap();
}
