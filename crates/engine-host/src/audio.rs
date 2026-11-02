//! Audio output.
//!
//! The engine mixes on its own threads and hands finished PCM to
//! `env.wasm_audio_publish`, which `audio-worklet.js` copied into an
//! `AudioWorkletProcessor`. Same contract here, minus the device: the host owns
//! the framing and validation, the sink decides what to do with the samples.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::host::{HostError, HostResult};

/// Largest block the engine may publish in one call (48 kHz stereo ≈ 85 ms).
/// Guards against a corrupt frame length turning into a huge allocation on the
/// consumer side.
pub const MAX_FRAMES: usize = 4096;

/// One interleaved PCM block — the payload of `env.wasm_audio_publish`.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioBlock {
    /// Mixer stream id (`0` = master, others are buses/stingers).
    pub stream: u32,
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u32,
    /// Interleaved `f32` samples; `frames * channels` long.
    pub samples: Vec<f32>,
}

impl AudioBlock {
    pub fn new(
        stream: u32,
        sample_rate: u32,
        channels: u16,
        samples: Vec<f32>,
    ) -> HostResult<Self> {
        if channels == 0 {
            return Err(HostError::InvalidArgument("zero audio channels".into()));
        }
        if sample_rate == 0 {
            return Err(HostError::InvalidArgument("zero sample rate".into()));
        }
        let stride = channels as usize;
        if samples.len() % stride != 0 {
            return Err(HostError::InvalidArgument(format!(
                "{} samples is not a multiple of {} channels",
                samples.len(),
                stride
            )));
        }
        let frames = samples.len() / stride;
        if frames > MAX_FRAMES {
            return Err(HostError::InvalidArgument(format!(
                "block of {frames} frames exceeds {MAX_FRAMES}"
            )));
        }
        Ok(Self {
            stream,
            sample_rate,
            channels,
            frames: u32::try_from(frames).unwrap_or(u32::MAX),
            samples,
        })
    }

    pub fn duration(&self) -> Duration {
        if self.sample_rate == 0 {
            return Duration::ZERO;
        }
        Duration::from_secs_f64(f64::from(self.frames) / f64::from(self.sample_rate))
    }

    /// Structural check for blocks built field-by-field from engine memory.
    pub fn validate(&self) -> HostResult<()> {
        if self.channels == 0 || self.sample_rate == 0 {
            return Err(HostError::InvalidArgument(
                "audio block has no channels or sample rate".into(),
            ));
        }
        let expected = self.frames as usize * self.channels as usize;
        if self.samples.len() != expected {
            return Err(HostError::InvalidArgument(format!(
                "audio block declares {expected} samples, carries {}",
                self.samples.len()
            )));
        }
        if self.frames as usize > MAX_FRAMES {
            return Err(HostError::InvalidArgument(format!(
                "audio block of {} frames exceeds {MAX_FRAMES}",
                self.frames
            )));
        }
        Ok(())
    }
}

/// `env.wasm_audio_publish` — the device (or its stand-in).
pub trait AudioSink: Send + Sync {
    /// Publish one block. Must not block the engine's mixer for long; sinks
    /// that need buffering copy and return.
    fn publish(&self, block: AudioBlock) -> HostResult<()>;
}

/// Sink for headless/server runs: validates and discards, counting what it saw
/// so a test can assert the mixer actually produced output.
#[derive(Debug, Default)]
pub struct NullAudioSink {
    blocks: AtomicU64,
    frames: AtomicU64,
}

impl NullAudioSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blocks(&self) -> u64 {
        self.blocks.load(Ordering::Relaxed)
    }

    pub fn frames(&self) -> u64 {
        self.frames.load(Ordering::Relaxed)
    }
}

impl AudioSink for NullAudioSink {
    fn publish(&self, block: AudioBlock) -> HostResult<()> {
        block.validate()?;
        self.blocks.fetch_add(1, Ordering::Relaxed);
        self.frames
            .fetch_add(u64::from(block.frames), Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_construction_validates_shape() {
        let block = AudioBlock::new(0, 48_000, 2, vec![0.0; 480]).expect("block");
        assert_eq!(block.frames, 240);
        assert!((block.duration().as_secs_f64() - 0.005).abs() < 1e-9);

        assert!(AudioBlock::new(0, 48_000, 0, vec![]).is_err());
        assert!(AudioBlock::new(0, 0, 2, vec![0.0; 4]).is_err());
        assert!(AudioBlock::new(0, 48_000, 2, vec![0.0; 3]).is_err());
        let too_big = vec![0.0; (MAX_FRAMES + 1) * 2];
        assert!(AudioBlock::new(0, 48_000, 2, too_big).is_err());
    }

    #[test]
    fn validate_catches_hand_built_mismatches() {
        let ok = AudioBlock {
            stream: 1,
            sample_rate: 44_100,
            channels: 1,
            frames: 3,
            samples: vec![0.1, 0.2, 0.3],
        };
        assert!(ok.validate().is_ok());

        let short = AudioBlock {
            samples: vec![0.1, 0.2],
            ..ok.clone()
        };
        assert!(matches!(
            short.validate(),
            Err(HostError::InvalidArgument(_))
        ));
        let overlong = AudioBlock {
            frames: (MAX_FRAMES + 1) as u32,
            ..ok
        };
        assert!(overlong.validate().is_err());
    }

    #[test]
    fn null_sink_counts_published_blocks() {
        let sink = NullAudioSink::new();
        let block = AudioBlock::new(2, 48_000, 2, vec![0.0; 512]).expect("block");
        sink.publish(block.clone()).expect("publish");
        sink.publish(block).expect("publish");
        assert_eq!(sink.blocks(), 2);
        assert_eq!(sink.frames(), 512);
    }
}