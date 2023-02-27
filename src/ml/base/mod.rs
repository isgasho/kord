//! Base types for machine learning.

pub mod data;
pub mod gather;
pub mod helpers;
pub mod mlp;
pub mod model;

use burn::config::Config;
use std::path::PathBuf;

/// The standard frequency space size to use across all ML operations.
///
/// This covers up to C9, which is beyond the range of a standard 88-key piano (C8).
pub const FREQUENCY_SPACE_SIZE: usize = 8192;

/// The standard mel space size to use across all ML operations.
pub const INPUT_SPACE_SIZE: usize = MEL_SPACE_SIZE + 128;

/// The standard mel space size to use across all ML operations.
pub const MEL_SPACE_SIZE: usize = 512;

/// The standard number of classes to use across all ML operations.
pub const NUM_CLASSES: usize = 128;

// Training configuration.

#[derive(Debug, Config)]
pub struct TrainConfig {
    /// The source directory for the gathered samples.
    pub source: String,
    /// The destination directory for the trained model.
    pub destination: String,
    /// The log directory for training.
    pub log: String,

    /// Simulation data set size.
    pub simulation_size: usize,

    /// The number of Multi Layer Perceptron (MLP) layers.
    pub mlp_layers: usize,
    /// The number of neurons in each Multi Layer Perceptron (MLP) layer.
    pub mlp_size: usize,
    /// The Multi Layer Perceptron (MLP) dropout rate.
    pub mlp_dropout: f64,

    /// The number of epochs to train for.
    pub model_epochs: usize,
    /// The number of samples to use per epoch.
    pub model_batch_size: usize,
    /// The number of workers to use for training.
    pub model_workers: usize,
    /// The seed used for training.
    pub model_seed: u64,

    /// The Adam optimizer learning rate.
    pub adam_learning_rate: f64,
    /// The Adam optimizer weight decay.
    pub adam_weight_decay: f64,
    /// The Adam optimizer beta1.
    pub adam_beta1: f32,
    /// The Adam optimizer beta2.
    pub adam_beta2: f32,
    /// The Adam optimizer epsilon.`
    pub adam_epsilon: f32,

    /// The "sigmoid strength" of the final pass.
    pub sigmoid_strength: f32,
}

/// A single kord sample.
///
/// This is a single sample of a kord, which is a set of notes played together.
#[derive(Clone, Debug)]
pub struct KordItem {
    pub path: PathBuf,
    pub frequency_space: [f32; FREQUENCY_SPACE_SIZE],
    pub label: u128,
}

impl Default for KordItem {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            frequency_space: [0.0; FREQUENCY_SPACE_SIZE],
            label: 0,
        }
    }
}