use std::cmp;
use anyhow_tauri::IntoTAResult;
use serde::Serialize;
use anyhow::{Result, bail};

#[derive(Debug, Serialize)]
struct Timings {
  presc: u8,
  scll: u8,
  sclh: u8,
  sdadel: u8,
  scldel: u8,
}

impl Timings {
  fn new(i2cclk: u32, freq: u32) -> Result<Self> {
      // Refer to RM0433 Rev 7 Figure 539 for setup and hold timing:
      //
      // t_I2CCLK = 1 / PCLK1
      // t_PRESC  = (PRESC + 1) * t_I2CCLK
      // t_SCLL   = (SCLL + 1) * t_PRESC
      // t_SCLH   = (SCLH + 1) * t_PRESC
      //
      // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
      // t_SCL ~= t_SYNC1 + t_SYNC2 + t_SCLL + t_SCLH
      let ratio = i2cclk / freq;

      // For the standard-mode configuration method, we must have a ratio of 4
      // or higher
      if ratio < 4 {
        bail!("The I2C PCLK must be at least 4 times the bus frequency!");
      }

      let (presc_reg, scll, sclh, sdadel, scldel) = if freq > 100_000 {
          // Fast-mode (Fm) or Fast-mode Plus (Fm+)
          // here we pick SCLL + 1 = 2 * (SCLH + 1)

          // Prescaler, 384 ticks for sclh/scll. Round up then subtract 1
          let presc_reg = ((ratio - 1) / 384) as u8;
          // ratio < 1200 by pclk 120MHz max., therefore presc < 16

          // Actual precale value selected
          let presc = (presc_reg + 1) as u32;

          let sclh = ((ratio / presc) - 3) / 3;
          let scll = (2 * (sclh + 1)) - 1;

          let (sdadel, scldel) = if freq > 400_000 {
              // Fast-mode Plus (Fm+)
              // See table in datsheet
              if i2cclk < 17_000_000 {
                bail!("I2C clock must be >= 17MHz for Fast-mode Plus");
              }

              let sdadel = i2cclk / 8_000_000 / presc;
              let scldel = i2cclk / 4_000_000 / presc - 1;

              (sdadel, scldel)
          } else {
              // Fast-mode (Fm)
              // See table in datsheet
              if i2cclk < 8_000_000 {
                bail!("I2C clock must be >= 8MHz for Fast-mode");
              }

              let sdadel = i2cclk / 4_000_000 / presc;
              let scldel = i2cclk / 2_000_000 / presc - 1;

              (sdadel, scldel)
          };

          (presc_reg, scll as u8, sclh as u8, sdadel as u8, scldel as u8)
      } else {
          // Standard-mode (Sm)
          // here we pick SCLL = SCLH
          // See table in datsheet
          if i2cclk < 2_000_000 {
            bail!("I2C clock must be >= 2MHz for Standard-mode");
          }

          // Prescaler, 512 ticks for sclh/scll. Round up then
          // subtract 1
          let presc = (ratio - 1) / 512;
          let presc_reg = cmp::min(presc, 15) as u8;

          // Actual prescale value selected
          let presc = (presc_reg + 1) as u32;

          let sclh = ((ratio / presc) - 2) / 2;
          let scll = sclh;

          // Speed check
          if sclh >= 256 {
            bail!("The I2C PCLK is too fast for this bus frequency!");
          }

          let sdadel = i2cclk / 2_000_000 / presc;
          let scldel = i2cclk / 500_000 / presc - 1;

          (presc_reg, scll as u8, sclh as u8, sdadel as u8, scldel as u8)
      };

      // Sanity check
      if presc_reg >= 16 {
        bail!("Prescaler value must be less than 16");
      }

      // Keep values within reasonable limits for fast per_ck
      let sdadel = cmp::max(sdadel, 2);
      let scldel = cmp::max(scldel, 4);

      //(presc_reg, scll, sclh, sdadel, scldel)
      Ok(Self {
          presc: presc_reg,
          scll,
          sclh,
          sdadel,
          scldel,
      })
  }
}

#[tauri::command]
fn get_timings(i2cclk: u32, freq: u32) -> anyhow_tauri::TAResult<Timings> {
  Timings::new(i2cclk, freq).into_ta_result()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_timings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
