pub mod tract;
pub mod transforms;
pub mod wav_utils;

use std::sync::Arc;
use num_complex::Complex32;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};

// Normalization constants
pub const MEAN_NORM_INIT: [f32; 2] = [-15., -15.];
pub const UNIT_NORM_INIT: [f32; 2] = [0.1, 0.1];

#[derive(Clone)]
pub struct DFState {
    pub sr: usize,
    pub window_size: usize,
    pub frame_size: usize,
    pub freq_size: usize,
    pub nb_erb: usize,
    pub min_nb_erb_freqs: usize,
    pub erb_fb: Vec<usize>,
    pub fft_forward: Arc<dyn RealToComplex<f32>>,
    pub fft_inverse: Arc<dyn ComplexToReal<f32>>,
    pub analysis_window: Vec<f32>,
    pub synthesis_window: Vec<f32>,
    pub analysis_mem: Vec<f32>,
    pub synthesis_mem: Vec<f32>,
    pub synthesis_mem2: Vec<f32>,
    pub norm_states: Vec<Vec<f32>>,
}

impl DFState {
    pub fn new(
        sr: usize,
        window_size: usize,
        frame_size: usize,
        nb_erb: usize,
        min_nb_erb_freqs: usize,
    ) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_forward = planner.plan_fft_forward(window_size);
        let fft_inverse = planner.plan_fft_inverse(window_size);
        let freq_size = window_size / 2 + 1;
        let analysis_window = vec![0.0; window_size];
        let synthesis_window = vec![0.0; window_size];
        let analysis_mem = vec![0.0; window_size - frame_size];
        let synthesis_mem = vec![0.0; window_size - frame_size];
        let synthesis_mem2 = vec![0.0; window_size - frame_size];
        let erb_fb = vec![0; nb_erb];
        
        DFState {
            sr,
            window_size,
            frame_size,
            freq_size,
            nb_erb,
            min_nb_erb_freqs,
            erb_fb,
            fft_forward,
            fft_inverse,
            analysis_window,
            synthesis_window,
            analysis_mem,
            synthesis_mem,
            synthesis_mem2,
            norm_states: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.analysis_mem.fill(0.0);
        self.synthesis_mem.fill(0.0);
        self.synthesis_mem2.fill(0.0);
    }

    pub fn init_norm_states(&mut self, nb_df: usize) {
        self.norm_states = vec![vec![0.1; nb_df]; 2];
    }

    pub fn analysis(&mut self, input: &[f32], output: &mut [Complex32]) {
        // Implementation would go here
    }

    pub fn synthesis(&mut self, input: &mut [Complex32], output: &mut [f32]) {
        // Implementation would go here
    }

    pub fn feat_erb(&mut self, input: &[Complex32], alpha: f32, output: &mut [f32]) {
        // Implementation would go here
    }

    pub fn feat_cplx(&mut self, input: &[Complex32], alpha: f32, output: &mut [Complex32]) {
        // Implementation would go here
    }

    pub fn apply_mask(&mut self, spec: &mut [Complex32], mask: &[f32]) {
        // Implementation would go here
    }
}

pub fn post_filter(noisy: &[Complex32], enh: &mut [Complex32], beta: f32) {
    // Implementation would go here
}

pub fn find_max<'a, I>(iter: I) -> Option<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    iter.into_iter().fold(None, |max, &val| {
        if val.is_nan() {
            max
        } else {
            match max {
                None => Some(val),
                Some(max_val) => Some(max_val.max(val)),
            }
        }
    })
}

pub fn find_max_abs<'a, I>(iter: I) -> Option<f32>
where
    I: IntoIterator<Item = &'a f32>,
{
    iter.into_iter().fold(None, |max, &val| {
        if val.is_nan() {
            max
        } else {
            match max {
                None => Some(val.abs()),
                Some(max_val) => Some(max_val.max(val.abs())),
            }
        }
    })
}

pub fn median(x: &mut [usize]) -> usize {
    if x.is_empty() {
        return 0;
    }
    x.sort_unstable();
    let mid = x.len() / 2;
    if x.len() % 2 == 0 {
        (x[mid - 1] + x[mid]) / 2
    } else {
        x[mid]
    }
}

pub fn band_compr(output: &mut [f32], input: &[f32], erb_fb: &[usize]) {
    // Implementation would go here
}

pub fn compute_band_corr(output: &mut [f32], input1: &[Complex32], input2: &[Complex32], erb_fb: &[usize]) {
    // Implementation would go here
}

pub fn apply_interp_band_gain(spec: &mut [Complex32], gain: &[f32], erb_fb: &[usize]) {
    // Implementation would go here
}

pub fn interp_band_gain(output: &mut [f32], gain: &[f32], erb_fb: &[usize]) {
    // Implementation would go here
}

pub fn band_mean_norm_erb(input: &mut [f32], state: &mut [f32], alpha: f32) {
    // Implementation would go here
}

pub fn band_unit_norm(input: &mut [Complex32], state: &mut [f32], alpha: f32) {
    // Implementation would go here
}

pub fn frame_analysis(input: &[f32], output: &mut [Complex32], state: &mut DFState) {
    // Implementation would go here
}

pub fn frame_synthesis(input: &mut [Complex32], output: &mut [f32], state: &mut DFState) {
    // Implementation would go here
}