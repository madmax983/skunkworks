//! # Neural Network Primitive
//!
//! This module provides a ground-up implementation of a Multi-Layer Perceptron (MLP)
//! neural network, powered by a custom matrix math engine.
//!
//! It includes:
//! - A `Matrix` struct for linear algebra operations.
//! - A `Network` struct for building, training, and running neural networks.
//! - Basic activation functions (sigmoid).
//!
//! # The "Why"
//!
//! This isn't `ndarray` or `torch`. It's a hand-crafted, zero-dependency (almost) implementation
//! designed to show *how* neural networks work under the hood. It exposes the raw math:
//! dot products, gradient descent, and backpropagation.
//!
//! # Example: Solving XOR
//!
//! Here is the "Hero's Journey" - training a network to solve the XOR problem:
//!
//! ```rust
//! use neuro_terminal::nn::Network;
//!
//! // Create a network with:
//! // - 2 input neurons
//! // - 4 hidden neurons
//! // - 1 output neuron
//! let mut nn = Network::new(vec![2, 4, 1], 0.5);
//!
//! // XOR Training Data
//! let inputs = vec![
//!     vec![0.0, 0.0],
//!     vec![0.0, 1.0],
//!     vec![1.0, 0.0],
//!     vec![1.0, 1.0],
//! ];
//!
//! let targets = vec![
//!     vec![0.0],
//!     vec![1.0],
//!     vec![1.0],
//!     vec![0.0],
//! ];
//!
//! // Train for a few iterations
//! for _ in 0..10_000 {
//!     for i in 0..inputs.len() {
//!         nn.train(&inputs[i], &targets[i]);
//!     }
//! }
//!
//! // Predict
//! let prediction = nn.predict(&[0.0, 1.0]);
//! assert!(prediction[0] > 0.9); // Should be close to 1.0
//! ```

use rand::Rng;

/// A lightweight Matrix implementation for neural network computations.
///
/// This struct holds a flat `Vec<f64>` and dimensions (`rows`, `cols`).
/// It supports fundamental linear algebra operations like dot products,
/// addition, and transposition.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    /// The number of rows in the matrix.
    pub rows: usize,
    /// The number of columns in the matrix.
    pub cols: usize,
    /// The flattened data storage (row-major).
    pub data: Vec<f64>,
}

impl Matrix {
    /// Creates a new Matrix with the given dimensions and data.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows.
    /// * `cols` - Number of columns.
    /// * `data` - A flat vector containing `rows * cols` elements.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` does not equal `rows * cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(m.rows, 2);
    /// assert_eq!(m.cols, 2);
    /// ```
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match rows*cols");
        Self { rows, cols, data }
    }

    /// Creates a new Matrix filled with zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let m = Matrix::zeros(2, 3);
    /// assert_eq!(m.data, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    /// ```
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Creates a new Matrix filled with random values between -1.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let m = Matrix::random(2, 2);
    /// for x in m.data {
    ///     assert!(x >= -1.0 && x < 1.0);
    /// }
    /// ```
    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..rows * cols).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Self { rows, cols, data }
    }

    /// Performs matrix multiplication (Dot Product).
    ///
    /// Returns a new Matrix representing `self * other`.
    ///
    /// # Panics
    ///
    /// Panics if `self.cols` does not equal `other.rows`.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(1, 2, vec![1.0, 2.0]);
    /// let b = Matrix::new(2, 1, vec![3.0, 4.0]);
    /// let c = a.dot(&b);
    ///
    /// assert_eq!(c.rows, 1);
    /// assert_eq!(c.cols, 1);
    /// assert_eq!(c.data[0], 1.0 * 3.0 + 2.0 * 4.0); // 11.0
    /// ```
    pub fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.cols, other.rows,
            "Matrix dimensions mismatch for dot product"
        );
        let mut result = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                result.data[i * result.cols + j] = sum;
            }
        }
        result
    }

    /// Performs element-wise addition.
    ///
    /// # Panics
    ///
    /// Panics if matrices do not have the same dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(2, 1, vec![1.0, 2.0]);
    /// let b = Matrix::new(2, 1, vec![10.0, 20.0]);
    /// let c = a.add(&b);
    ///
    /// assert_eq!(c.data, vec![11.0, 22.0]);
    /// ```
    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Performs element-wise subtraction.
    ///
    /// # Panics
    ///
    /// Panics if matrices do not have the same dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(2, 1, vec![10.0, 20.0]);
    /// let b = Matrix::new(2, 1, vec![1.0, 2.0]);
    /// let c = a.sub(&b);
    ///
    /// assert_eq!(c.data, vec![9.0, 18.0]);
    /// ```
    pub fn sub(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Performs element-wise multiplication (Hadamard product).
    ///
    /// Note: This is NOT matrix multiplication. Use `dot` for that.
    ///
    /// # Panics
    ///
    /// Panics if matrices do not have the same dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(2, 1, vec![2.0, 3.0]);
    /// let b = Matrix::new(2, 1, vec![4.0, 5.0]);
    /// let c = a.mul(&b);
    ///
    /// assert_eq!(c.data, vec![8.0, 15.0]);
    /// ```
    pub fn mul(&self, other: &Matrix) -> Matrix {
        // Element-wise multiplication (Hadamard product)
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Multiplies every element by a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(2, 1, vec![1.0, 2.0]);
    /// let b = a.mul_scalar(10.0);
    ///
    /// assert_eq!(b.data, vec![10.0, 20.0]);
    /// ```
    pub fn mul_scalar(&self, scalar: f64) -> Matrix {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Applies a function to every element in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(2, 1, vec![1.0, -1.0]);
    /// let b = a.map(|x| x.abs());
    ///
    /// assert_eq!(b.data, vec![1.0, 1.0]);
    /// ```
    pub fn map(&self, func: fn(f64) -> f64) -> Matrix {
        let data = self.data.iter().map(|&x| func(x)).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Returns a new Matrix that is the transpose of this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let a = Matrix::new(1, 2, vec![1.0, 2.0]); // 1 row, 2 cols
    /// let b = a.transpose();
    ///
    /// assert_eq!(b.rows, 2);
    /// assert_eq!(b.cols, 1);
    /// assert_eq!(b.data, vec![1.0, 2.0]);
    /// ```
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j * result.cols + i] = self.data[i * self.cols + j];
            }
        }
        result
    }

    /// Creates a column vector (Matrix with 1 column) from a Vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    ///
    /// let m = Matrix::from_vec(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(m.rows, 3);
    /// assert_eq!(m.cols, 1);
    /// ```
    pub fn from_vec(data: Vec<f64>) -> Matrix {
        Matrix::new(data.len(), 1, data)
    }
}

/// The Sigmoid activation function.
///
/// Maps any input to the range (0.0, 1.0).
/// Formula: `1 / (1 + e^-x)`
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// A simple Feed-Forward Neural Network (Multi-Layer Perceptron).
///
/// The network consists of:
/// - Layers defined by neuron counts.
/// - Weights matrices connecting layers.
/// - Bias matrices for each layer.
pub struct Network {
    /// The architecture of the network (e.g., `vec![2, 3, 1]`).
    pub layers: Vec<usize>,
    /// Weights between layers. `weights[0]` connects layer 0 to layer 1.
    pub weights: Vec<Matrix>,
    /// Biases for each layer (except input).
    pub biases: Vec<Matrix>,
    /// Activations for each layer stored during forward pass.
    pub data: Vec<Matrix>,
    /// Unactivated inputs (z) for each layer, stored for backprop.
    pub z_data: Vec<Matrix>,
    /// The learning rate for gradient descent.
    pub learning_rate: f64,
}

impl Network {
    /// Creates a new Network with random weights and biases.
    ///
    /// # Arguments
    ///
    /// * `layers` - A vector representing the number of neurons in each layer.
    ///   E.g., `vec![2, 4, 1]` creates 2 input, 4 hidden, 1 output.
    /// * `learning_rate` - The step size for gradient descent (e.g., 0.1).
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Network;
    /// let nn = Network::new(vec![2, 3, 1], 0.1);
    /// assert_eq!(nn.layers.len(), 3);
    /// ```
    pub fn new(layers: Vec<usize>, learning_rate: f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Self {
            layers,
            weights,
            biases,
            data: vec![],
            z_data: vec![],
            learning_rate,
        }
    }

    /// Performs a forward pass through the network and stores intermediate state.
    ///
    /// This is used during training because it saves the activations needed for
    /// backpropagation.
    ///
    /// Returns the output layer's activations.
    pub fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs.to_vec());
        self.data = vec![inputs.clone()];
        self.z_data = vec![];

        let mut current = inputs;

        for i in 0..self.weights.len() {
            let z = self.weights[i].dot(&current).add(&self.biases[i]);
            self.z_data.push(z.clone());
            current = z.map(sigmoid);
            self.data.push(current.clone());
        }

        current.data
    }

    /// Predicts the output for the given inputs without storing state.
    ///
    /// Use this for inference after the network is trained.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Network;
    /// let nn = Network::new(vec![2, 1], 0.1);
    /// let output = nn.predict(&[0.5, 0.5]);
    /// assert_eq!(output.len(), 1);
    /// ```
    pub fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs.to_vec());
        let mut current = inputs;

        for i in 0..self.weights.len() {
            let z = self.weights[i].dot(&current).add(&self.biases[i]);
            current = z.map(sigmoid);
        }

        current.data
    }

    /// Trains the network using a single training example.
    ///
    /// This performs:
    /// 1. Forward pass (calculating output).
    /// 2. Error calculation (Target - Output).
    /// 3. Backpropagation (calculating gradients).
    /// 4. Weight update (Stochastic Gradient Descent).
    ///
    /// # Arguments
    ///
    /// * `inputs` - The input vector.
    /// * `targets` - The expected output vector.
    pub fn train(&mut self, inputs: &[f64], targets: &[f64]) {
        // Forward pass
        self.forward(inputs);

        let targets = Matrix::from_vec(targets.to_vec());
        let mut errors = targets.sub(self.data.last().unwrap());

        for i in (0..self.weights.len()).rev() {
            let outputs = &self.data[i + 1];
            let prev_outputs = &self.data[i];

            // Gradient = error * sigmoid_derivative(outputs)
            // sigmoid_derivative(z) = sigmoid(z) * (1 - sigmoid(z)) = outputs * (1 - outputs)
            let d_outputs = outputs.map(|x| x * (1.0 - x));
            let gradients = errors.mul(&d_outputs).mul_scalar(self.learning_rate);

            // Calculate deltas
            let weight_deltas = gradients.dot(&prev_outputs.transpose());

            // Store old weights for error propagation
            let old_weights = &self.weights[i];
            let next_errors = old_weights.transpose().dot(&errors);

            // Update weights and biases
            self.weights[i] = self.weights[i].add(&weight_deltas);
            self.biases[i] = self.biases[i].add(&gradients);

            errors = next_errors;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_dot() {
        let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::new(2, 2, vec![2.0, 0.0, 1.0, 2.0]);
        let c = a.dot(&b);
        assert_eq!(c.data, vec![4.0, 4.0, 10.0, 8.0]);
    }

    #[test]
    fn test_xor() {
        let mut nn = Network::new(vec![2, 4, 1], 0.5);

        let inputs = [
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];

        let targets = [vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

        for _ in 0..20000 {
            let idx = rand::random::<usize>() % 4;
            nn.train(&inputs[idx], &targets[idx]);
        }

        for i in 0..4 {
            let out = nn.forward(&inputs[i]);
            let target = targets[i][0];
            // println!(
            //     "In: {:?}, Target: {}, Out: {:.4}",
            //     inputs[i], target, out[0]
            // );
            assert!(
                (out[0] - target).abs() < 0.2,
                "Failed to learn XOR: In {:?}, Expected {}, Got {}",
                inputs[i],
                target,
                out[0]
            );
        }
    }
}
