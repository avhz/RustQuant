use nalgebra::{DMatrix, DVector};

use RustQuant::ml::*;

fn main() -> Result<(), LinearRegressionError> {
    // TEST DATA GENERATED FROM THE FOLLOWING R CODE:
    //
    // set.seed(2023)
    //
    // features    <- c("x1", "x2", "x3")
    //
    // x_train     <- data.frame(matrix(rnorm(12), 4, 3)); colnames(x_train) <- features
    // x_test      <- data.frame(matrix(rnorm(12), 4, 3)); colnames(x_test)  <- features
    //
    // response    <- rnorm(4)
    //
    // (model <- lm(response ~ ., data = x_train))
    // (preds <- predict(model, newdata = x_test))

    // Create some training and test data (X).
    #[rustfmt::skip]
    let x_train = DMatrix::from_row_slice(
        4, // rows
        3, // columns
        &[-0.083784355, -0.63348570, -0.39926660, 
          -0.982943745,  1.09079746, -0.46812305,
          -1.875067321, -0.91372727,  0.32696208,
          -0.186144661,  1.00163971, -0.41274690],
    );

    #[rustfmt::skip]
    let x_test = DMatrix::from_row_slice(
        4, // rows
        3, // columns
        &[0.56203647, 0.59584645, -0.41165301, 
          0.66335826, 0.45209183, -0.29432715,
         -0.60289728, 0.89674396, 1.21857396, 
          0.69837769, 0.57221651, 0.24411143],
    );

    // Create the response vector (Y).
    let response = DVector::from_row_slice(&[-0.44515196, -1.84780364, -0.62882531, -0.86108069]);

    // Create the input object for the linear regression model.
    let input = LinearRegressionInput::with_response(x_train, &response, InputClass::Train);

    // Fit the model to the training data.
    // You need to specify which method (decomposition) to use for fitting the model.
    // The available methods are:
    //     - `None`: No decomposition is used.
    //     - `QR`: QR decomposition is used.
    //     - `SVD`: SVD decomposition is used.
    let output = input.fit(Decomposition::QR)?;

    // Predict the response for the test data.
    let preds = output.predict(x_test)?;

    // Print the results.
    println!("Intercept: {:?}", output.intercept);
    println!("Coefficients: {:?}", output.coefficients);
    println!("Predictions: {:?}", preds);
    Ok(())
}
