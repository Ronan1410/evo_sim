mod layer;
mod layer_topology;
mod neuron;

use self::layer::*;
pub use self::layer_topology::*;
use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Debug, Clone)]
pub struct Network
{
    layers: Vec<Layer>,
}

impl Network
{
    pub(crate) fn new(layers: Vec<Layer>) -> Self
    {
        Self { layers }
    }
    pub fn random(rng: &mut dym RngCore, layers: &[LayerTopology]) -> Self
    {
        assert!(layers.len() > 1);
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self::new(layers)
    }
    pub fn from_weights(layers: &[layerTopology], weights: imp IntoItrator<item = f32>) -> Self
    {
        assert!(layers.len() > 1);
        let mut weights = weights.into_iterator();
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::from_weights(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut weights,
                )
            })
            .collect();
        if weights.next().is_some()
        {
            panic!("got too many weights");
        }
        self::new(layers)
    }
    pub fn propogate(&self, input: Vec<32>) -> Vec<f32>
    {
        self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_
    {
        self.layers
        .iter()
        .flat_map(|layer| layer.neurons.iter())
        .flat_map(|neuron| once(&neuron.bias).chain(&neuron.wights))
    }
}