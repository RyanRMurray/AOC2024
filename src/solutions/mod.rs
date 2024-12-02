mod day01;
mod day02;
pub mod templates;

use anyhow::Result;

/// Add new solutions to this const
pub const SOLUTIONS: [fn(&str) -> Result<f32>; 2] = [day01::day01, day02::day02];
