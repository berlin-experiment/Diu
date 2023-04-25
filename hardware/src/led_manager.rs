use core::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::*;

pub struct LedManager<'a> {
    nr_leds: u8,
    current_leds: Vec<Led>,
    target_leds: Vec<Led>,
    tx: TxRmtDriver<'a>,
    t0h: Pulse,
    t0l: Pulse,
    t1h: Pulse,
    t1l: Pulse,
    signal: FixedLengthSignal<48>,
}

enum ValueComparison {
    greater,
    lesser,
    equal,
}

fn compare_values(value: f32, comparison: f32) -> ValueComparison {
    if value > comparison {
        return ValueComparison::greater;
    }
    if value < comparison {
        return ValueComparison::lesser;
    }
    return ValueComparison::equal;
}

impl LedManager<'_> {
    pub fn new(nr_leds: u8) -> Self {
        let mut led_vec = Vec::new();
        let mut target_led_vec: Vec<Led> = Vec::new();
        let peripherals = Peripherals::take().unwrap();

        // Onboard RGB LED pin
        // ESP32-C3-DevKitC-02 gpio8, esp-rs gpio2
        let led_pin = peripherals.pins.gpio2;

        let channel = peripherals.rmt.channel0;
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, led_pin, &config).unwrap();

        let ticks_hz = tx.counter_clock().unwrap();
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350)).unwrap();
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800)).unwrap();
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700)).unwrap();
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600)).unwrap();
        let signal = FixedLengthSignal::<48>::new();

        for i in 0..nr_leds {
            led_vec.push(Led {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
                brightness_increment: 0.01,
            })
        }

        for i in 0..nr_leds {
            target_led_vec.push(Led {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
                brightness_increment: 0.01,
            })
        }

        LedManager {
            nr_leds: nr_leds,
            current_leds: led_vec,
            target_leds: target_led_vec,
            tx: tx,
            t0h: t0h,
            t0l: t0l,
            t1h: t1h,
            t1l: t1l,
            signal: signal,
        }
    }

    pub fn turn_on(&mut self) {
        for target in self.target_leds.iter_mut() {
            target.alpha = 1.0;
        }
    }

    pub fn turn_off(&mut self) {
        for target in self.target_leds.iter_mut() {
            target.alpha = 0.0;
        }
    }

    pub fn set_all_rgb(&mut self, red: f32, green: f32, blue: f32) {
        for target in self.target_leds.iter_mut() {
            target.set_rgb(red, green, blue);
        }
    }

    pub fn set_increment(&mut self, increment: f32) {
        for led in self.current_leds.iter_mut() {
            led.brightness_increment = increment;
        }
    }

    pub fn update(&mut self) {
        let mut signal = VariableLengthSignal::new();

        for (i, led) in self.current_leds.iter_mut().enumerate() {
            let diff_alpha: f32 = led.alpha - self.target_leds[i].alpha;
            let diff_red: f32 = led.red - self.target_leds[i].red;
            let diff_green: f32 = led.green - self.target_leds[i].green;
            let diff_blue: f32 = led.blue - self.target_leds[i].blue;

            match compare_values(diff_alpha, 0.0) {
                ValueComparison::greater => {
                    led.set_brightness(led.alpha - led.brightness_increment)
                }
                ValueComparison::lesser => led.set_brightness(led.alpha + led.brightness_increment),
                ValueComparison::equal => {}
            }

            match compare_values(diff_red, 0.0) {
                ValueComparison::greater => {
                    led.set_rgb(led.red - led.brightness_increment, led.green, led.blue)
                }
                ValueComparison::lesser => {
                    led.set_rgb(led.red + led.brightness_increment, led.green, led.blue)
                }
                ValueComparison::equal => {}
            }

            match compare_values(diff_green, 0.0) {
                ValueComparison::greater => {
                    led.set_rgb(led.red, led.green - led.brightness_increment, led.blue)
                }
                ValueComparison::lesser => {
                    led.set_rgb(led.red, led.green + led.brightness_increment, led.blue)
                }
                ValueComparison::equal => {}
            }

            match compare_values(diff_blue, 0.0) {
                ValueComparison::greater => {
                    led.set_rgb(led.red, led.green, led.blue - led.brightness_increment)
                }
                ValueComparison::lesser => {
                    led.set_rgb(led.red, led.green, led.blue + led.brightness_increment)
                }
                ValueComparison::equal => {}
            }

            // Send values to leds
            let color: u32 = (((led.green * led.alpha) as u8 as u32) << 16)
                | (((led.red * led.alpha) as u8 as u32) << 8)
                | (led.blue * led.alpha) as u8 as u32;

            for j in (0..24).rev() {
                let p = 2_u32.pow(j);
                let bit = p & color != 0;
                let (high_pulse, low_pulse) = if bit {
                    (self.t1h, self.t1l)
                } else {
                    (self.t0h, self.t0l)
                };
                signal
                    .push(vec![&high_pulse, &low_pulse].into_iter())
                    .unwrap();
                // self.signal
                //     .set((23 * i) - j as usize, &(high_pulse, low_pulse))
                //     .unwrap();
            }
        }
        self.tx.start_blocking(&signal).unwrap();
    }
}

struct Led {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    brightness_increment: f32,
}

impl Led {
    fn set_rgb(&mut self, mut red: f32, mut green: f32, mut blue: f32) {
        // sanitise u8 values
        if red > 255.0 {
            red = 255.0;
        }
        if red < 0.0 {
            red = 0.0;
        }
        if green > 255.0 {
            green = 255.0;
        }
        if green < 0.0 {
            green = 0.0;
        }
        if blue > 255.0 {
            blue = 255.0;
        }
        if blue < 0.0 {
            blue = 0.0;
        }

        self.red = red;
        self.green = green;
        self.blue = blue;
    }

    fn set_brightness(&mut self, mut alpha: f32) {
        if alpha > 1.0 {
            alpha = 1.0;
        }
        if alpha < 0.0 {
            alpha = 0.0;
        }
        self.alpha = alpha;
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}
