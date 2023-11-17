use rustfft::{FftPlanner, num_complex::Complex};

let mut planner = FftPlanner::<f32>::new();
let fft = planner.plan_fft_forward(1234);

let mut buffer = vec![Complex{ re: 0.0, im: 0.0}; 1234];

fft.process(&mut buffer);