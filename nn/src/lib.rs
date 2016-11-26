extern crate leaf;

use leaf::layer::{Layer, LayerConfig};
use leaf::layers::common::linear::LinearConfig;

fn main() {
    // construct the config for a fully connected layer with 500 notes
    let linear_1: LayerConfig = LayerConfig::new("linear1", LinearConfig { output_size: 500 });
}
