#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Instant;
use embassy_time::Timer;

use defmt as _;
use defmt::info;
use galactic_unicorn_embassy::pins::UnicornSensorPins;
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics_core::{pixelcolor::Rgb888, prelude::Point};

use unicorn_graphics::UnicornGraphics;

use galactic_unicorn_embassy::pins::UnicornDisplayPins;
use galactic_unicorn_embassy::GalacticUnicorn;
use galactic_unicorn_embassy::{HEIGHT, WIDTH};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting Cosmic Unicorn");

    let p = embassy_rp::init(Default::default());

    info!("Init done.");
    let display_pins = UnicornDisplayPins {
        column_clock: p.PIN_13,
        column_data: p.PIN_14,
        column_latch: p.PIN_15,
        column_blank: p.PIN_16,
        row_bit_0: p.PIN_17,
        row_bit_1: p.PIN_18,
        row_bit_2: p.PIN_19,
        row_bit_3: p.PIN_20,
    };

    let sensor_pins = UnicornSensorPins {
        light_sensor: p.PIN_28,
    };

    info!("Starting Cosmic Unicorn");
    let mut gu = GalacticUnicorn::new(p.PIO0, display_pins, sensor_pins, p.ADC, p.DMA_CH0);

    let mut graphics = UnicornGraphics::<WIDTH, HEIGHT>::new();
    let mut heat: [[f32; HEIGHT + 2]; WIDTH] = [[0.0; HEIGHT + 2]; WIDTH];

    gu.set_pixels(&graphics);
    gu.set_brightness(100);

    loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let coord = Point {
                    x: x as i32,
                    y: y as i32,
                };

                let x = x as usize;
                let y = y as usize;
                if heat[x][y] > 0.5 {
                    let color = Rgb888::new(255, 255, 180);
                    graphics.set_pixel(coord, color);
                } else if heat[x][y] > 0.4 {
                    let color = Rgb888::new(220, 160, 0);
                    graphics.set_pixel(coord, color);
                } else if heat[x][y] > 0.3 {
                    let color = Rgb888::new(180, 50, 0);
                    graphics.set_pixel(coord, color);
                } else if heat[x][y] > 0.2 {
                    let color = Rgb888::new(40, 40, 40);
                    graphics.set_pixel(coord, color);
                }

                // Update this pixel by averaging the below pixels
                if x == 0 {
                    heat[x][y] =
                        (heat[x][y] + heat[x][y + 2] + heat[x][y + 1] + heat[x + 1][y + 1]) / 4.0;
                } else if x == WIDTH - 1 {
                    heat[x][y] =
                        (heat[x][y] + heat[x][y + 2] + heat[x][y + 1] + heat[x - 1][y + 1]) / 4.0;
                } else {
                    heat[x][y] = (heat[x][y]
                        + heat[x][y + 2]
                        + heat[x][y + 1]
                        + heat[x - 1][y + 1]
                        + heat[x + 1][y + 1])
                        / 5.0;
                }

                heat[x][y] -= 0.01;
                heat[x][y] = heat[x][y].max(0.0);
            }
        }

        gu.set_pixels(&graphics);

        // clear the bottom row and then add a new fire seed to it
        for x in 0..WIDTH {
            heat[x][HEIGHT] = 0.0;
        }

        // add a new random heat source
        for _ in 0..5 {
            let ticks = Instant::now().as_ticks();
            let px: usize = ticks as usize % 30 + 1;
            heat[px][HEIGHT] = 1.0;
            heat[px + 1][HEIGHT] = 1.0;
            heat[px - 1][HEIGHT] = 1.0;
            heat[px][32] = 1.0;
            heat[px + 1][HEIGHT + 1] = 1.0;
            heat[px - 1][HEIGHT + 1] = 1.0;
        }

        Timer::after_millis(50).await;
    }
}
