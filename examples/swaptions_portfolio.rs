// // DO NOT USE THIS YET. IT IS NOT FINISHED.

// #![allow(non_snake_case)]

// use RustQuant::autodiff::*;

fn main() {}
//     let g = Graph::new();

//     let delta = 0.25;

//     let nopt = 15;
//     let maturities = vec![4, 4, 4, 8, 8, 8, 20, 20, 20, 28, 28, 28, 40, 40, 40];
//     let swaprates = vec![
//         0.045, 0.05, 0.055, 0.045, 0.05, 0.055, 0.045, 0.05, 0.055, 0.045, 0.05, 0.055, 0.045,
//         0.05, 0.055,
//     ];

//     let mut nmat = 40;
//     let mut n = nmat + 40;

//     let mut l = vec![0.0; n];
//     let mut l_b = vec![0.0; n];
//     let mut l_b2 = vec![0.0; n];
//     let mut v_d = vec![0.0; 2 * n];
//     let mut v_d2 = vec![0.0; 2 * n];
//     let mut l_d = vec![0.0; 2 * n * n];
//     let mut l2 = vec![0.0; n * n];
//     let mut l3 = vec![0.0; n * n];

//     // let mut fl: Vec<F<f64, 80>> = vec![F::default(); n];
//     // let mut bl: Vec<B<f64>> = vec![B::default(); n];
//     // let mut bl2: Vec<B<f64>> = vec![B::default(); n];

//     let mut lambda = vec![0.0; n];
//     let mut lambda_b = vec![0.0; n];
//     let mut z = vec![0.0; n];

//     for i in 0..n {
//         l[i] = 0.05;
//     }

//     // portfolio_b(
//     //     n,
//     //     nmat,
//     //     delta,
//     //     nopt,
//     //     &maturities,
//     //     &swaprates,
//     //     &mut l,
//     //     &mut l_b,
//     //     &mut v,
//     // );

//     for i in 0..n * n {
//         l_d[i] = 0.0;
//     }

//     for i in 0..n {
//         l_d[i + i * n] = 1.0;
//     }

//     // portfolio_d(
//     //     n,
//     //     nmat,
//     //     delta,
//     //     nopt,
//     //     &maturities,
//     //     &swaprates,
//     //     &mut l,
//     //     &mut l_d,
//     //     &mut v,
//     //     &mut v_d,
//     // );

//     // for i in 0..n {
//     //     bl[i] = 0.05;
//     // }

//     // portfolio(
//     //     n,
//     //     nmat,
//     //     delta,
//     //     nopt,
//     //     &maturities,
//     //     &swaprates,
//     //     &mut bl,
//     //     &mut bv,
//     // );
//     // bv.diff(0, 1);

//     // for i in 0..n {
//     //     fl[i] = 0.05;
//     //     fl[i].diff(i);
//     // }

//     // portfolio(
//     //     n,
//     //     nmat,
//     //     delta,
//     //     nopt,
//     //     &maturities,
//     //     &swaprates,
//     //     &mut fl,
//     //     &mut fv,
//     // );

//     println!("\n portfolio sensitivity check \n\n");
//     // for i in 0..n {
//     //     println!(" {}  {}  {}  {}", l_b[i], v_d[i], bl[i].d(0), fv.d(i));
//     // }

//     for i in 0..n {
//         lambda[i] = 0.2;
//     }
//     for i in 0..nmat {
//         z[i] = 0.3;
//     }

//     let mut tt: Vec<time::Duration> = Vec::with_capacity(8);

//     for pass in 0..8 {
//         let start = time::Instant::now();

//         let mut npath = match pass {
//             0 => 1,
//             1..=3 => 100000,
//             4..=5 => 10000,
//             6..=7 => 100000,
//             _ => panic!("Unexpected pass value: {}", pass),
//         };

//         // for path in 0..npath {
//         //     // AD reverse mode
//         //     if pass == 0 || pass == 4 {
//         //         for i in 0..N {
//         //             BL[i] = 0.05;
//         //             BL2[i] = BL[i];
//         //         }
//         //         path_calc(N, Nmat, delta, &mut BL2, &lambda, &z);
//         //         portfolio(
//         //             N,
//         //             Nmat,
//         //             delta,
//         //             Nopt,
//         //             &maturities,
//         //             &swaprates,
//         //             &mut BL2,
//         //             &mut Bv,
//         //         );
//         //         for i in 0..N {
//         //             BL2[i] = 0.0;
//         //         }
//         //         Bv.diff(0, 1);
//         //     }

//         //     // AD forward mode
//         //     if pass == 0 || pass == 5 {
//         //         for i in 0..N {
//         //             FL[i] = 0.05;
//         //             FL[i].diff(i);
//         //         }
//         //         path_calc(N, Nmat, delta, &mut FL, &lambda, &z);
//         //         portfolio(
//         //             N,
//         //             Nmat,
//         //             delta,
//         //             Nopt,
//         //             &maturities,
//         //             &swaprates,
//         //             &mut FL,
//         //             &mut Fv,
//         //         );
//         //     }

//         //     // hybrid reverse mode
//         //     if pass == 0 || pass == 6 {
//         //         for i in 0..N {
//         //             L[i] = 0.05;
//         //         }
//         //         path_calc_b1(N, Nmat, delta, &mut L, &lambda, &z, &mut L2, &mut L3);

//         //         for i in 0..N {
//         //             BL2[i] = L[i];
//         //         }
//         //         portfolio(
//         //             N,
//         //             Nmat,
//         //             delta,
//         //             Nopt,
//         //             &maturities,
//         //             &swaprates,
//         //             &mut BL2,
//         //             &mut Bv,
//         //         );
//         //         Bv.diff(0, 1);

//         //         for i in 0..N {
//         //             L_b2[i] = BL2[i].d(0);
//         //         }
//         //         path_calc_b2(
//         //             N,
//         //             Nmat,
//         //             delta,
//         //             &mut L,
//         //             &mut L_b2,
//         //             &lambda,
//         //             &mut lambda_b,
//         //             &z,
//         //             &mut L2,
//         //             &mut L3,
//         //         );
//         //     }

//         //     // hybrid forward mode
//         //     if pass == 0 || pass == 7 {
//         //         for i in 0..N {
//         //             L[i] = 0.05;
//         //         }
//         //         for i in 0..N * N {
//         //             L_d[i] = 0.0;
//         //         }
//         //         for i in 0..N {
//         //             L_d[i + i * N] = 1.0;
//         //         }
//         //         path_calc_d(N, Nmat, delta, &mut L, &mut L_d, &lambda, &z);

//         //         for i in 0..N {
//         //             FL[i] = L[i];
//         //             FL[i].diff(i);
//         //         }
//         //         portfolio(
//         //             N,
//         //             Nmat,
//         //             delta,
//         //             Nopt,
//         //             &maturities,
//         //             &swaprates,
//         //             &mut FL,
//         //             &mut Fv2,
//         //         );

//         //         for i in 0..N {
//         //             v_d2[i] = 0.0;
//         //             for i2 in 0..N {
//         //                 v_d2[i] += L_d[i + i2 * N] * Fv2.d(i2);
//         //             }
//         //         }
//         //     }
//         // }

//         // if pass == 0 {
//         //     println!("\n path + portfolio sensitivity check \n\n");
//         //     for i in 0..n {
//         //         println!(
//         //             " {}  {}  {}  {}  {}  {}",
//         //             l_b[i],
//         //             v_d[i],
//         //             bl[i].d(0),
//         //             fv.d(i),
//         //             l_b2[i],
//         //             v_d2[i]
//         //         );
//         //     }
//         // }

//         let end = start.elapsed();

//         tt[pass] = end;
//         // if pass > 0 {
//         //     tt[pass - 1] = (tt[pass] - tt[pass - 1]) * (1000.0 / npath as f64);
//         // }
//     }

//     println!("\n timings in milliseconds per path\n");
//     println!("   val  hc-b  hc-f   BAD   FAD  hyb-b hyb-f \n");
//     // println!(
//     //     " {} {} {} {} {} {} {}",
//     //     tt[0], tt[1], tt[2], tt[3], tt[4], tt[5], tt[6]
//     // );

//     // let N = 10;
//     // let Nmat = 10;
//     // let delta = 0.25;
//     // let mut L = g.vars(&vec![0.05; N]);
//     // // let mut L = vec![g.var(0.05); N];
//     // let lambda = vec![0.01; N - 1];
//     // let z = vec![0.01; Nmat];

//     // path_calc(&g, N, Nmat, delta, &mut L, &lambda, &z);

//     // println!("{:?}", L);

//     // let adjoints = L[0].accumulate();

//     // println!("{:?}", adjoints.wrt(&L));
// }

// // Monte Carlo LIBOR path calculation
// fn path_calc<'v>(
//     graph: &'v Graph,
//     N: usize,
//     Nmat: usize,
//     delta: f64,
//     L: &mut [Variable<'v>],
//     lambda: &[f64],
//     z: &[f64],
// ) {
//     let mut i: usize;
//     let mut n: usize;
//     let mut sqez: f64;
//     let mut lam: f64;
//     let mut con1: f64;
//     let mut v: Variable<'v>;
//     let mut vrat: Variable<'v>;

//     for n in 0..Nmat {
//         sqez = (delta * z[n]).sqrt();
//         v = graph.var(0.0);

//         for i in (n + 1)..N {
//             lam = lambda[i - n - 1];
//             con1 = delta * lam;
//             v = v + con1 * L[i] / (1.0 + delta * L[i]);
//             vrat = (con1 * v + lam * (sqez - 0.5 * con1)).exp();
//             L[i] = L[i] * vrat;
//         }
//     }
// }

// // Forward pathwise sensitivity calculation of deltas
// // (commented out bits are for vega calculation)

// fn path_calc_d(
//     N: usize,
//     Nmat: usize,
//     delta: f64,
//     L: &mut [f64],
//     L_d: &mut [f64],
//     lambda: &[f64],
//     z: &[f64],
// ) {
//     let mut i: usize;
//     let mut k: usize;
//     let mut n: usize;
//     let mut N2: usize;
//     let mut sqez: f64;
//     let mut v: f64;
//     let mut lam: f64;
//     let mut faci: f64;
//     let mut vrat: f64;
//     let mut con1: f64;
//     let mut con2: f64;
//     let mut con3: f64;

//     let mut v1 = vec![0.0; N];
//     let mut v2 = vec![0.0; N];
//     let mut v_l = vec![0.0; N];

//     N2 = N * N;

//     for n in 0..Nmat {
//         sqez = (delta).sqrt() * z[n];

//         v = 0.0;

//         for k in 0..N {
//             v1[k] = 0.0;
//             v2[k] = 0.0;
//         }

//         for i in (n + 1)..N {
//             lam = lambda[i - n - 1];
//             con1 = delta * lam;
//             faci = 1.0 / (1.0 + delta * L[i]);
//             v += con1 * L[i] * faci;
//             // v_l[i-n] = delta*L[i]*faci;
//             vrat = (con1 * (-0.5 * lam + v) + lam * sqez).exp();

//             con2 = con1 * faci * faci;
//             con3 = con1 * L[i] * vrat;

//             for k in 0..(i + 1) {
//                 v1[k] += con2 * L_d[k + i * N];
//                 L_d[k + i * N] = L_d[k + i * N] * vrat + con3 * v1[k];
//                 // v2[k]        += con2*L_d[k+i*N+N2];
//                 // L_d[k+i*N+N2] = L_d[k+i*N+N2]*vrat + con3*v2[k];
//             }

//             /*
//             for (k=0;k<=i-n-1; k++) {
//             L_d[k+i*N+N2] += con3*v_l[k];
//             }
//             L_d[(i-n-1)+i*N+N2] += L[i]*vrat*((v-lam)*delta+sqez);
//             */
//             L[i] = L[i] * vrat;
//         }
//     }
// }

// //    void path_calc_d(const int N, const int Nmat, const double delta,
// //     double L[], double L_d[],
// //     const double lambda[], const double z[])
// // {
// // int    i, k, n, N2;
// // double sqez, v, lam, faci, vrat, con1, con2, con3;

// // double v1[N], v2[N], v_l[N];
// // // vector<double> v1(N), v2(N), v_l(N);

// // N2 = N*N;

// // for(n=0; n<Nmat; n++) {
// // sqez = sqrt(delta)*z[n];

// // v = 0;
// // for (k=0; k<N; k++) {
// // v1[k]=0;
// // v2[k]=0;
// // }

// // for (i=n+1; i<N; i++) {
// // lam  = lambda[i-n-1];
// // con1 = delta*lam;
// // faci = 1.0/(1.0+delta*L[i]);
// // v   += con1*L[i]*faci;
// // // v_l[i-n] = delta*L[i]*faci;
// // vrat = exp(con1*(-0.5*lam+v) + lam*sqez);

// // con2 = con1*faci*faci;
// // con3 = con1*L[i]*vrat;

// // for (k=0; k<i+1; k++) {
// // v1[k]        += con2*L_d[k+i*N];
// // L_d[k+i*N]    = L_d[k+i*N]*vrat + con3*v1[k];
// // // v2[k]        += con2*L_d[k+i*N+N2];
// // // L_d[k+i*N+N2] = L_d[k+i*N+N2]*vrat + con3*v2[k];
// // }

// // /*
// // for (k=0;k<=i-n-1; k++) {
// // L_d[k+i*N+N2] += con3*v_l[k];
// // }
// // L_d[(i-n-1)+i*N+N2] += L[i]*vrat*((v-lam)*delta+sqez);
// // */
// // L[i] = L[i]*vrat;
// // }
// // }
// // }
