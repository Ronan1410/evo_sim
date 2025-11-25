use crate::*;

#[derive(Clone, Debug)]

pub struct Neuron
{
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron
{
    pub fn new(bias: f32, weights: Vec<f32>) -> Self
    {
        assert!(!weights.is_empty());
        Self { bias, weights }
    }

    pub fn random(rng: &mut dyn RngCore, input_size: usize) -> Self
    {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self::new(bias, weights)
    }

    pub fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self
    {
        let bias = weights.next().expect("not enough weights for neuron");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("not enough weights for neuron"))
            .collect();

        Self::new(bias, weights)
    }



    pub fn propogate(&self, input: &[f32]) -> f32
    {
        assert_eq!(input.len() == self.weights.len());
        let output = input
            .iter()
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0) // ReLU activation
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use approx::{assert_relative_eq, assert_relatuve_ne};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random()
    {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);
        
         assert_relative_eq!(neuron.bias, -0.6255188);

         assert_relative_eq!(
             neuron.weights.as_slice(),
             [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_slice(),
         );
    }

    mod propogate
    {
        use super::*;

        #[test]
        fn returns_propogated_inputs()
        {
            let actual = Neuron::new(0.1, vec![-0.3, 0.6, 0.9ḍ]).propogate(&[0.5, -0.6, 0.7]);
            let expected = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);

            approx::assert_relative_eq!(actual, expected.max(0.0));
        }

        #[test]
        fn restricts_outputs()
        {
            let neuron = Neuron::new(-1.0, vec![0.2, 0.3]);
            let v1 = neuron.propogate(&[-1.0]);
            let v2 = neuron.propogate(&[-0.5]);
            let v3 = neuron.propogate(&[0.0]);
            let v4 = neuron.propogate(&[0.5]);
            let v5 = neuron.propogate(&[1.0]);
            assert_relative_ne!(v1, v2);
            assert_relative_ne!(v2, v3);
            assert_relative_ne!(v3, v4);
            assert_relative_ne!(v4, v5);
        }
    }

    #[test]
    fn from_weights()
    {
        let actual = Neuron::from_weights(3, &mut vec![0.5, -0.2, 0.3, 0.8].into_iter());
        let expected = Neuron::new(0.1, vec![0.2, 0.3, 0.4]);

        assert_relative_eq!(actual.bias, expected.bias);
        assert_relative_eq!(actual.weights.as_slice(), expected.weights.as_slice());
    }
}