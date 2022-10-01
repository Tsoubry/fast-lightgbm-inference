// Implemented based on: https://github.com/vaaaaanquish/lightgbm-rs/blob/main/src/booster.rs

use libc::{c_char, c_double, c_longlong, c_void};
use std;
use std::ffi::CString;

use lightgbm_sys;

use crate::error::{Error, Result};

macro_rules! lgbm_call {
    ($x:expr) => {
        Error::check_return_value(unsafe { $x })
    };
}

pub struct BoosterPredictor {
    handle: lightgbm_sys::BoosterHandle,
}

impl BoosterPredictor {
    fn new(handle: lightgbm_sys::BoosterHandle) -> Self {
        BoosterPredictor { handle }
    }

    pub fn from_string(model_data: &str) -> Result<Self> {
        let model_str = CString::new(model_data).unwrap();
        let mut out_num_iterations = 0;
        let mut handle = std::ptr::null_mut();
        lgbm_call!(lightgbm_sys::LGBM_BoosterLoadModelFromString(
            model_str.as_ptr() as *const c_char,
            &mut out_num_iterations,
            &mut handle
        ))?;

        Ok(BoosterPredictor::new(handle))
    }

    pub fn from_file(filename: &str) -> Result<Self> {
        let filename_str = CString::new(filename).unwrap();
        let mut out_num_iterations = 0;
        let mut handle = std::ptr::null_mut();
        lgbm_call!(lightgbm_sys::LGBM_BoosterCreateFromModelfile(
            filename_str.as_ptr() as *const c_char,
            &mut out_num_iterations,
            &mut handle
        ))?;

        Ok(BoosterPredictor::new(handle))
    }

    pub fn predict(&self, data: Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>> {
        let data_length = data.len();
        let feature_length = data[0].len();
        let params = CString::new("").unwrap();
        let mut out_length: c_longlong = 0;
        let flat_data = data.into_iter().flatten().collect::<Vec<_>>();

        // get num_class
        let mut num_class = 0;
        lgbm_call!(lightgbm_sys::LGBM_BoosterGetNumClasses(
            self.handle,
            &mut num_class
        ))?;

        let out_result: Vec<f64> = vec![Default::default(); data_length * num_class as usize];

        lgbm_call!(lightgbm_sys::LGBM_BoosterPredictForMat(
            self.handle,
            flat_data.as_ptr() as *const c_void,
            lightgbm_sys::C_API_DTYPE_FLOAT64 as i32,
            data_length as i32,
            feature_length as i32,
            1_i32,
            0_i32,
            0_i32,
            -1_i32,
            params.as_ptr() as *const c_char,
            &mut out_length,
            out_result.as_ptr() as *mut c_double
        ))?;

        // reshape for multiclass [1,2,3,4,5,6] -> [[1,2,3], [4,5,6]]  # 3 class
        let reshaped_output = if num_class > 1 {
            out_result
                .chunks(num_class as usize)
                .map(|x| x.to_vec())
                .collect()
        } else {
            vec![out_result]
        };
        Ok(reshaped_output)
    }

    // Do this with pure Rust
    // pub fn save_file(&self, filename: &str) -> Result<()> {
    //     let filename_str = CString::new(filename).unwrap();
    //     lgbm_call!(lightgbm_sys::LGBM_BoosterSaveModel(
    //         self.handle,
    //         0_i32,
    //         -1_i32,
    //         0_i32,
    //         filename_str.as_ptr() as *const c_char
    //     ))?;
    //     Ok(())
    // }
}

impl Drop for BoosterPredictor {
    fn drop(&mut self) {
        lgbm_call!(lightgbm_sys::LGBM_BoosterFree(self.handle)).unwrap();
    }
}

// unsafe send/sync is fine, we're only reading from it for predictions
unsafe impl Send for BoosterPredictor {}

unsafe impl Sync for BoosterPredictor {}
