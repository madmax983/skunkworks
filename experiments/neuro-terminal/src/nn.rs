use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match rows*cols");
        Self { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..rows * cols).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Self { rows, cols, data }
    }

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

    pub fn mul_scalar(&self, scalar: f64) -> Matrix {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn map(&self, func: fn(f64) -> f64) -> Matrix {
        let data = self.data.iter().map(|&x| func(x)).collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j * result.cols + i] = self.data[i * self.cols + j];
            }
        }
        result
    }

    pub fn from_vec(data: Vec<f64>) -> Matrix {
        Matrix::new(data.len(), 1, data)
    }
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct Network {
    pub layers: Vec<usize>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub data: Vec<Matrix>,   // Activations for each layer (including input)
    pub z_data: Vec<Matrix>, // Unactivated inputs (z) for each layer
    pub learning_rate: f64,
}

impl Network {
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

    pub fn forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs);
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

    pub fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        let inputs = Matrix::from_vec(inputs);
        let mut current = inputs;

        for i in 0..self.weights.len() {
            let z = self.weights[i].dot(&current).add(&self.biases[i]);
            current = z.map(sigmoid);
        }

        current.data
    }

    pub fn train(&mut self, inputs: Vec<f64>, targets: Vec<f64>) {
        // Forward pass
        self.forward(inputs);

        let targets = Matrix::from_vec(targets);
        let mut errors = targets.sub(self.data.last().unwrap());

        for i in (0..self.weights.len()).rev() {
            let outputs = &self.data[i + 1];
            let prev_outputs = &self.data[i];

            // Gradient = error * sigmoid_derivative(outputs)
            // sigmoid_derivative(z) = sigmoid(z) * (1 - sigmoid(z)) = outputs * (1 - outputs)
            let d_outputs = outputs.map(|x| x * (1.0 - x));
            let gradients = errors.mul(&d_outputs);
            let gradients = gradients.mul_scalar(self.learning_rate);

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
            nn.train(inputs[idx].clone(), targets[idx].clone());
        }

        for i in 0..4 {
            let out = nn.forward(inputs[i].clone());
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
