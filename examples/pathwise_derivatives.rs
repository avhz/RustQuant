use RustQuant::autodiff::*;
use RustQuant::stochastics::*;

fn main() {
    let g = Graph::new();

    let drift = 0.05;
    let diffusion = 0.9;

    // Differentiate the payoff function with respect to the spot price.
    let spot = g.var(150.);
    let strike = 100.;
    let payoff = payoff(spot, strike);
    let gradient = payoff.accumulate();

    println!("payoff = {}", payoff.value);
    println!("d(payoff)/d(spot) = {}", gradient.wrt(&spot));
    println!("grad = {:?}", gradient.wrt(&[spot]));

    // Generate a random path of spot prices.
    let gbm = GeometricBrownianMotion::new(drift, diffusion);
    let path = gbm.euler_maruyama(150., 0., 1., 1000, 1, false).paths[0].clone();

    println!("spot = {}", path.last().unwrap());

    // Generate multiple paths of spot prices.
    let gbm = GeometricBrownianMotion::new(drift, diffusion);
    let paths = gbm.euler_maruyama(150., 0., 1., 1000, 10, false).paths;

    // Compute pathwise derivatives.
    // let spot = g.var(150.);
    // let strike = 100.;
    // let payoff = payoff(spot, strike);
    // let gradient = payoff.accumulate();
}

// Discount factor.
fn df<'v>(rate: Variable<'v>, time: Variable<'v>) -> Variable<'v> {
    (-rate * time).exp()
}

fn payoff<'v>(spot: Variable<'v>, strike: f64) -> Variable<'v> {
    RustQuant::autodiff::Max::max(&(spot - strike), 0.)
}

// type Matrix<'v> = Vec<Vec<Variable<'v>>>;
// type Vector<'v> = Vec<Variable<'v>>;

struct VariableMatrix<'v> {
    data: Vec<Vec<Variable<'v>>>,
}

struct VariableVector<'v> {
    data: Vec<Variable<'v>>,
}

impl<'v> std::ops::Add<VariableMatrix<'v>> for VariableMatrix<'v> {
    type Output = VariableMatrix<'v>;

    fn add(self, rhs: VariableMatrix<'v>) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());
        assert_eq!(self.data[0].len(), rhs.data[0].len());

        let mut data = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            let mut row = Vec::with_capacity(self.data[0].len());

            for j in 0..self.data[0].len() {
                row.push(self.data[i][j] + rhs.data[i][j]);
            }

            data.push(row);
        }

        Self::Output { data }
    }
}

impl<'v> std::ops::Add<VariableVector<'v>> for VariableVector<'v> {
    type Output = VariableVector<'v>;

    fn add(self, rhs: VariableVector<'v>) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());

        let mut data = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            data.push(self.data[i] + rhs.data[i]);
        }

        Self::Output { data }
    }
}
