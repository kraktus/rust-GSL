//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

//! A __Rust__ binding for the [GSL library][] (the GNU Scientific Library).
//!
//! ## Installation
//!
//! This binding requires the [GSL library] library to be installed.
//!
//! This crate works with Cargo and is on [crates.io].  Just add the
//! following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! GSL = "*"
//! ```
//!
//! Add the following line to your source code:
//!
//! ```rust
//! extern crate rgsl;
//! ```
//!
//! ## Documentation
//!
//! You can access the __rgsl__ documentation locally, just build it:
//!
//! ```Shell
//! > cargo doc --open
//! ```
//!
//! Then open this file with an internet browser:
//! `file:///{rgsl_location}/target/doc/rgsl/index.html`
//!
//! You can also access the latest build of the documentation via the internet
//! [here](https://docs.rs/crate/GSL/).
//!
//! ## License
//! __rust-GSL__ is a wrapper for __GSL__, therefore inherits the
//! [GPL license](http://www.gnu.org/copyleft/gpl.html).
//!
//! [crates.io]: https://crates.io/crates/GSL
//! [GSL library]: http://www.gnu.org/software/gsl/
//!
//! Here is the list of all modules :

#![crate_name = "rgsl"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_unsafe)]

extern crate c_vec;
extern crate gsl_sys as sys;
extern crate libc;

pub use types::{
    ChebSeries, Combination, ComplexF32, ComplexF64, CquadWorkspace, DiscreteHankel,
    EigenGenHermVWorkspace, EigenGenHermWorkspace, EigenGenSymmVWorkspace, EigenGenSymmWorkspace,
    EigenGenVWorkspace, EigenGenWorkspace, EigenHermitianVWorkspace, EigenHermitianWorkspace,
    EigenNonSymmVWorkspace, EigenNonSymmWorkspace, EigenSymmetricVWorkspace,
    EigenSymmetricWorkspace, FftComplexWaveTable, FftComplexWorkspace, GLFixedTable, Histogram,
    Histogram2D, Histogram2DPdf, HistogramPdf, IntegrationFixedType, IntegrationFixedWorkspace,
    IntegrationQawoTable, IntegrationQawsTable, IntegrationWorkspace, Interp, InterpAccel,
    InterpType, LevinUTruncWorkspace, LevinUWorkspace, MatrixComplexF32, MatrixComplexF64,
    MatrixF32, MatrixF64, MatrixView, Minimizer, MinimizerType, MiserMonteCarlo, MiserParams,
    MultiFitFdfSolver, MultiFitFdfSolverType, MultiFitFunction, MultiFitFunctionFdf, MultiSet,
    ODEiv2Control, ODEiv2Driver, ODEiv2Evolve, ODEiv2Step, ODEiv2StepType, ODEiv2System,
    Permutation, PlainMonteCarlo, PolyComplex, QRng, QRngType, ReadNTuples, Result, ResultE10, Rng,
    RngType, RootFSolver, RootFSolverType, RootFdfSolver, RootFdfSolverType, SimAnnealing,
    SimAnnealingParams, Spline, VectorComplexF32, VectorComplexF64, VectorF32, VectorF64,
    VectorView, VegasMonteCarlo, VegasParams, Wavelet, WaveletType, WaveletWorkspace, WriteNTuples,
};

pub use elementary::Elementary;
pub use pow::Pow;
pub use trigonometric::Trigonometric;
pub use types::rng;
pub use utilities::IOStream;

// enums part
pub use self::enums::{
    EigenSort, FftDirection, GaussKonrodRule, IntegrationQawo, Mode, ODEiv, SfLegendreNorm, Value,
    VegasMode, WaveletDirection,
};

mod enums;
mod macros;
mod utilities;

#[doc(hidden)]
pub mod ffi;

pub mod randist;
pub mod types;

pub mod airy;
pub mod bessel;
pub mod blas;
pub mod cblas;
pub mod clausen;
pub mod coulomb;
pub mod coupling_coefficients;
pub mod dawson;
pub mod debye;
pub mod dilogarithm;
pub mod eigen;
pub mod elementary;
pub mod elementary_operations;
pub mod elliptic;
pub mod error;
pub mod exponential;
pub mod exponential_integrals;
pub mod fermi_dirac;
pub mod fft;
pub mod fit;
pub mod gamma_beta;
pub mod gegenbauer;
pub mod hypergeometric;
pub mod integration;
pub mod interpolation;
pub mod jacobian_elliptic;
pub mod laguerre;
pub mod lambert_w;
pub mod legendre;
pub mod linear_algebra;
pub mod logarithm;
pub mod minimizer;
pub mod multifit;
pub mod multilarge;
pub mod multilinear;
pub mod numerical_differentiation;
pub mod physical_constant;
pub mod polynomials;
pub mod pow;
pub mod power;
pub mod psi;
pub mod roots;
pub mod sort;
pub mod statistics;
pub mod synchrotron;
pub mod transport;
pub mod trigonometric;
pub mod util;
pub mod wavelet_transforms;
pub mod zeta;

/// The maximum x such that gamma(x) is not considered an overflow.
pub static SF_GAMMA_XMAX: f64 = 171.0;
/// The maximum n such that gsl_sf_fact(n) does not give an overflow.
pub static SF_FACT_NMAX: f64 = 170.0;
/// The maximum n such that gsl_sf_doublefact(n) does not give an overflow.
pub static SF_DOUBLEFACT_NMAX: f64 = 297.0;

pub static SF_MATHIEU_COEFF: u32 = 100;

pub static DBL_EPSILON: f64 = 2.2204460492503131e-16;
pub static SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-08;
pub static ROOT3_DBL_EPSILON: f64 = 6.0554544523933429e-06;
pub static ROOT4_DBL_EPSILON: f64 = 1.2207031250000000e-04;
pub static ROOT5_DBL_EPSILON: f64 = 7.4009597974140505e-04;
pub static ROOT6_DBL_EPSILON: f64 = 2.4607833005759251e-03;

pub static DBL_MIN: f64 = 2.2250738585072014e-308;
pub static SQRT_DBL_MIN: f64 = 1.4916681462400413e-154;
pub static ROOT3_DBL_MIN: f64 = 2.8126442852362996e-103;
pub static ROOT4_DBL_MIN: f64 = 1.2213386697554620e-77;
pub static ROOT5_DBL_MIN: f64 = 2.9476022969691763e-62;
pub static ROOT6_DBL_MIN: f64 = 5.3034368905798218e-52;

pub static DBL_MAX: f64 = ::std::f64::MAX; //1.7976931348623156e+308;
pub static SQRT_DBL_MAX: f64 = 1.3407807929942596e+154;
pub static ROOT3_DBL_MAX: f64 = 5.6438030941222897e+102;
pub static ROOT4_DBL_MAX: f64 = 1.1579208923731620e+77;
pub static ROOT5_DBL_MAX: f64 = 4.4765466227572707e+61;
pub static ROOT6_DBL_MAX: f64 = 2.3756689782295612e+51;
pub static LOG_DBL_MAX: f64 = 7.0978271289338397e+02;

pub static NAN: f64 = 0f64 / 0f64;
pub static POSINF: f64 = 1f64 / 0f64;
pub static NEGINF: f64 = -1f64 / 0f64;
