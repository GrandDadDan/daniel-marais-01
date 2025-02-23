use rand::{Rng};
use burn::tensor::{Tensor, TensorData, Device};
use burn_ndarray::NdArray;
use burn::nn::{Linear, LinearConfig};
use textplots::{Chart, Plot, Shape};

struct LinearRegression {
    layer: Linear<NdArray>,
}

impl LinearRegression {
    fn new() -> Self {
        let device = Device::<NdArray>::default();
        let layer = LinearConfig::new(1, 1).init(&device);
        Self { layer }
    }

    fn forward(&self, x: Tensor<NdArray, 2>) -> Tensor<NdArray, 2> {
        self.layer.forward(x)
    }
}

fn generate_data(n: usize) -> (Tensor<NdArray, 2>, Tensor<NdArray, 2>) {
    let device = Device::<NdArray>::default(); // Define CPU device
    let mut rng = rand::rng();

    // Generate random x values
    let x: Vec<f32> = (0..n).map(|_| rng.random_range(0.0..10.0)).collect();
    // Generate corresponding y values (y = 2x + 1 with some noise)
    let y: Vec<f32> = x.iter().map(|&x| 2.0 * x + 1.0 + rng.random_range(-1.0..1.0)).collect();

    // Convert Vec<f32> into TensorData and then into Tensor
    let x_tensor = Tensor::<NdArray, 2>::from_data(TensorData::from(x), &device).reshape([n, 1]);
    let y_tensor = Tensor::<NdArray, 2>::from_data(TensorData::from(y), &device).reshape([n, 1]);

    (x_tensor, y_tensor)
}

fn calculate_loss(y_pred: Tensor<NdArray, 2>, y_true: Tensor<NdArray, 2>) -> Tensor<NdArray, 1> {
    let diff = y_pred - y_true;  // Element-wise difference
    let squared_diff = diff.clone().mul(diff);  // Square the differences
    squared_diff.mean()  // Mean of squared differences (MSE)
}

fn train() -> LinearRegression {
    //let device = Device::<NdArray>::default();
    let mut model = LinearRegression::new();
    let learning_rate = 0.01; // Set the learning rate

    // Generate Training Data
    let (x_train, y_train) = generate_data(100);

    for epoch in 0..1000 {
        // Forward pass
        let y_pred = model.forward(x_train.clone());

        // Calculate loss
        let loss = calculate_loss(y_pred.clone(), y_train.clone());

        // Manually compute the gradient for the loss (this assumes backward() is available)
        let grad_loss = loss.backward();  // Compute gradients

        // Now update the weights manually
        // We will assume that the Linear layer has weights and biases accessible
        // You need to update model parameters manually like this:

        // Example: Accessing and updating weights manually (simplified)
        let weight_gradient = grad_loss.clone(); // Replace with correct gradient computation
        let bias_gradient = grad_loss.clone();   // Replace with correct gradient computation

        // Update the model parameters manually
        // Assuming you have access to the model’s parameters
        model.layer.weight -= learning_rate * weight_gradient;  // Update weights
        model.layer.bias -= learning_rate * bias_gradient;      // Update bias

        // Print Loss for Monitoring
        if epoch % 100 == 0 {
            println!("Epoch: {}, Loss: {:?}", epoch, loss);
        }
    }

    model
}
fn evaluate(model: &LinearRegression) {
    let (x_test, y_test) = generate_data(20);
    let y_pred = model.forward(x_test.clone()).into_data().convert::<Vec<f32>>();

    let data: Vec<(f32, f32)> = x_test.into_data().convert::<Vec<f32>>()
        .into_iter().zip(y_pred.into_iter()).collect();

    Chart::new(100, 30, 0.0, 10.0)
        .lineplot(&Shape::Lines(&data))
        .display();
}
fn main() {
    let model = train();
    evaluate(&model);
}
