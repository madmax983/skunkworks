//! # Neural Network Core
//!
//! This module implements the core components of a simple Feed-Forward Neural Network,
//! including a custom [`Matrix`] struct for mathematical operations and the [`Network`]
//! struct itself, capable of forward propagation and backpropagation.

use rand::Rng;
use std::ops::{Add, Mul, Sub};

/// A 2D Matrix used for weights, biases, and activations.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// The flat data array storing the matrix elements in row-major order.
    pub data: Vec<f64>,
}

impl Matrix {
    /// Creates a new matrix from a flat vector of data.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    /// let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(m.rows, 2);
    /// assert_eq!(m.cols, 2);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the length of `data` does not equal `rows * cols`.
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match rows*cols");
        Self { rows, cols, data }
    }

    /// Creates a new matrix initialized with zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    /// let m = Matrix::zeros(2, 3);
    /// assert_eq!(m.get(1, 2), 0.0);
    /// ```
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Creates a new matrix initialized with random values between -1.0 and 1.0.
    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..rows * cols).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Self { rows, cols, data }
    }

    /// Gets an element at the specified row and column.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    /// Gets a mutable reference to an element at the specified row and column.
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.data[row * self.cols + col]
    }

    /// Performs matrix multiplication (dot product) with another matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Matrix;
    /// let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// let b = Matrix::new(2, 1, vec![2.0, 0.0]);
    /// let c = a.dot(&b);
    /// assert_eq!(c.get(0, 0), 2.0);
    /// assert_eq!(c.get(1, 0), 6.0);
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
                    sum += self.get(i, k) * other.get(k, j);
                }
                *result.get_mut(i, j) = sum;
            }
        }
        result
    }

    /// Adds another matrix to this one (element-wise).
    pub fn add(&self, other: &Matrix) -> Matrix {
        self + other
    }

    /// Subtracts another matrix from this one (element-wise).
    pub fn sub(&self, other: &Matrix) -> Matrix {
        self - other
    }

    /// Multiplies this matrix with another one (element-wise, Hadamard product).
    pub fn mul(&self, other: &Matrix) -> Matrix {
        self * other
    }

    /// Multiplies all elements in the matrix by a scalar value.
    pub fn mul_scalar(&self, scalar: f64) -> Matrix {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Maps a function over all elements of the matrix, returning a new matrix.
    pub fn map<F>(&self, func: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        let data = self.data.iter().map(|&x| func(x)).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Transposes the matrix, swapping rows and columns.
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                *result.get_mut(j, i) = self.get(i, j);
            }
        }
        result
    }

    /// Converts a 1D vector into an N x 1 column matrix.
    pub fn from_vec(data: Vec<f64>) -> Matrix {
        Matrix::new(data.len(), 1, data)
    }
}

impl<'b> Add<&'b Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &'b Matrix) -> Matrix {
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
}

impl<'b> Sub<&'b Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &'b Matrix) -> Matrix {
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
}

impl<'b> Mul<&'b Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &'b Matrix) -> Matrix {
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
}

/// The sigmoid activation function.
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// A simple Feed-Forward Neural Network.
#[derive(Clone)]
pub struct Network {
    /// The sizes of each layer (e.g., `[2, 5, 1]` for 2 inputs, 5 hidden, 1 output).
    pub layers: Vec<usize>,
    /// The weight matrices connecting the layers.
    pub weights: Vec<Matrix>,
    /// The bias matrices for each layer (except input).
    pub biases: Vec<Matrix>,
    /// Activations for each layer (including input).
    pub data: Vec<Matrix>,
    /// Unactivated inputs (z) for each layer.
    pub z_data: Vec<Matrix>,
    /// The learning rate used during backpropagation.
    pub learning_rate: f64,
}

impl Network {
    /// Creates a new neural network with the specified layer structure and learning rate.
    ///
    /// Weights and biases are initialized randomly.
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

    /// Performs a forward pass, storing activations and unactivated inputs internally.
    pub fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs.to_vec());
        self.data = vec![inputs.clone()];
        self.z_data = vec![];

        let mut current = inputs;

        for i in 0..self.weights.len() {
            let z = &self.weights[i].dot(&current) + &self.biases[i];
            self.z_data.push(z.clone());
            current = z.map(sigmoid);
            self.data.push(current.clone());
        }

        current.data
    }

    /// Predicts the output for a given input without storing intermediate state.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_terminal::nn::Network;
    /// let nn = Network::new(vec![2, 3, 1], 0.1);
    /// let prediction = nn.predict(&[0.5, -0.5]);
    /// assert_eq!(prediction.len(), 1);
    /// ```
    pub fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs.to_vec());
        let mut current = inputs;

        for i in 0..self.weights.len() {
            let z = &self.weights[i].dot(&current) + &self.biases[i];
            current = z.map(sigmoid);
        }

        current.data
    }

    /// Trains the network on a single input-target pair using backpropagation.
    ///
    /// Returns the Mean Squared Error (MSE) for the given pair.
    pub fn train(&mut self, inputs: &[f64], targets: &[f64]) -> f64 {
        // Forward pass
        self.forward(inputs);

        let targets = Matrix::from_vec(targets.to_vec());
        let mut errors = &targets - self.data.last().unwrap();

        // Calculate MSE
        let mse = errors.data.iter().map(|e| e.powi(2)).sum::<f64>() / errors.data.len() as f64;

        for i in (0..self.weights.len()).rev() {
            errors = self.backward_pass_layer(i, errors);
        }

        mse
    }

    /// Performs a single layer of backpropagation.
    fn backward_pass_layer(&mut self, layer_idx: usize, errors: Matrix) -> Matrix {
        let outputs = &self.data[layer_idx + 1];
        let prev_outputs = &self.data[layer_idx];

        // Gradient = error * sigmoid_derivative(outputs)
        // sigmoid_derivative(z) = sigmoid(z) * (1 - sigmoid(z)) = outputs * (1 - outputs)
        let d_outputs = outputs.map(|x| x * (1.0 - x));
        let gradients = (&errors * &d_outputs).mul_scalar(self.learning_rate);

        // Calculate deltas
        let weight_deltas = gradients.dot(&prev_outputs.transpose());

        // Store old weights for error propagation
        let old_weights = &self.weights[layer_idx];
        let next_errors = old_weights.transpose().dot(&errors);

        // Update weights and biases
        self.weights[layer_idx] = &self.weights[layer_idx] + &weight_deltas;
        self.biases[layer_idx] = &self.biases[layer_idx] + &gradients;

        next_errors
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
            println!(
                "In: {:?}, Target: {}, Out: {:.4}",
                inputs[i], target, out[0]
            );
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
