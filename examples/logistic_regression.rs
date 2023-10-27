// Many thanks to `s3bru` for this example.
// This example was authored by `s3bru` as a unit test for the
// `ml::regression::logistic_regression` implementation.

// Run: cargo run --release --example logistic_regression

use nalgebra::{DMatrix, DVector};
use time::Instant;
use RustQuant::ml::*;

fn main() {
    // The test generates sample data in the following way:
    //      1. For each of the N samples (train/test) draw K feature values each
    //          from a uniform distribution over (-1.,1.) and arrange as design matrix "X".
    //      2. For the coefficients of the generating distribution draw K values
    //          from surface of the unit sphere S_(K-1)  and a bias from uniform(-0.5,0.5);
    //          arrange as DVector "coefs"
    //      3. Compute vector of probabilities(target=1) as sigmoid(X_ext * coefs)
    //      4. Compute target values:for each sample i draw from Bernoulli(prob_i)

    use rand::prelude::*;
    use rand_distr::{Bernoulli, StandardNormal, Uniform};

    let n_train = 500; // Number of training samples
    let n_test = 80; // Number of test samples
    let k = 2; // Number of Features

    // Generate random coefficients which will be used to generate target values
    // for the x_i (direction uniform from sphere, bias uniform between -0.5 and 0.5 )
    // scaled by steepness.
    let it_normal = rand::thread_rng().sample_iter(StandardNormal).take(k);
    let bias = rand::thread_rng().sample(Uniform::new(-0.5, 0.5));
    let steepness = rand::thread_rng().sample(Uniform::new(1., 5.));
    let coefs = DVector::<f64>::from_iterator(k, it_normal)
        .normalize()
        .insert_row(0, bias)
        .scale(steepness);

    // Generate random design matrix for train/test
    let distr_uniform = Uniform::new(-1., 1.);
    let it_uniform_train = rand::thread_rng()
        .sample_iter(distr_uniform)
        .take(n_train * k);
    let x_train = DMatrix::<f64>::from_iterator(n_train, k, it_uniform_train);
    let it_uniform_test = rand::thread_rng()
        .sample_iter(distr_uniform)
        .take(n_test * k);
    let x_test = DMatrix::<f64>::from_iterator(n_test, k, it_uniform_test);

    // Extend each feature vector by 1. so that coefs_train[0] acts as bias.
    let x_train_extended = x_train.clone().insert_column(0, 1.0);
    let x_test_extended = x_test.clone().insert_column(0, 1.0);

    let eta_train = &x_train_extended * &coefs;
    let eta_test = &x_test_extended * &coefs;

    // Compute probabilities for each sample x_i.
    let probs_train = ActivationFunction::logistic(&eta_train);
    let probs_test = ActivationFunction::logistic(&eta_test);

    // Sample from Bernoulli distribution with p=p_i for each sample i.
    let y_train = probs_train
        .map(|p| Bernoulli::new(p).unwrap().sample(&mut rand::thread_rng()) as i32 as f64);
    let y_test = probs_test
        .map(|p| Bernoulli::new(p).unwrap().sample(&mut rand::thread_rng()) as i32 as f64);

    // Fit the model to the training data.
    let input = LogisticRegressionInput::with_response(x_train, &y_train, InputClass::Train);

    let start = Instant::now();
    let output = input.fit(LogisticRegressionAlgorithm::IRLS, f64::EPSILON.sqrt());
    let elapsed = start.elapsed();

    match output {
        Ok(output) => {
            let eta_hat = &x_test_extended * &output.coefficients;

            let y_hat =
                ActivationFunction::logistic(&eta_hat).map(|p| if p > 0.5 { 1. } else { 0. });

            let missclassification_rate = (y_hat - y_test).abs().sum() / n_test as f64;

            println!(
                "number of samples N_train={}, N_test={}, number of Features K={}",
                n_train, n_test, k
            );
            println!(
                "missclassification_rate(out of sample): \t{}",
                missclassification_rate
            );
            println!("Iterations: \t{}", output.iterations);
            println!("Time taken: \t{:?}", elapsed);
            println!("Coefficients found by IRLS:\n{:?}", &output.coefficients);
            println!(
                "Coefficients used for the generation of the training data:\n{:?}",
                &coefs
            );
        }
        Err(err) => {
            panic!("Failed to fit logistic regression model: {}", err);
        }
    }
}
