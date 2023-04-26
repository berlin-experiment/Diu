use core::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::*;

pub struct LedManager<'a> {
    nr_leds: u8,
    current_theme: ColorTheme,
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
    Greater,
    Lesser,
    Equal,
}

fn compare_values(value: f32, comparison: f32) -> ValueComparison {
    if value > comparison {
        return ValueComparison::Greater;
    }
    if value < comparison {
        return ValueComparison::Lesser;
    }
    return ValueComparison::Equal;
}

#[derive(Clone, Copy)]
pub enum ColorTheme {
    NoLight,
    Sunrise,
    Noon,
    Sunset,
    Moonlight,
}

// Here's a brief explanation of the changes:

// SUNRISE_COLORS: I chose warmer colors that resemble the golden hues of a sunrise.
// NOON_COLORS: I kept the colors bright and added a subtle blue to resemble a clear sky at noon.
// SUNSET_COLORS: I adjusted the colors to represent the warm and vivid tones of a sunset.
// MOONLIGHT_COLORS: I selected cooler shades of blue to represent the soft, ambient light of the moon.

const NO_LIGHT_COLORS: [(f32, f32, f32); 2] = [(0.0, 0.0, 0.0), (0.0, 0.0, 0.0)];

const SUNRISE_COLORS: [(f32, f32, f32); 2] = [(255.0, 120.0, 0.0), (245.0, 10.0, 10.0)];

const NOON_COLORS: [(f32, f32, f32); 2] = [(245.0, 255.0, 240.0), (255.0, 255.0, 240.0)];

const SUNSET_COLORS: [(f32, f32, f32); 2] = [(230.0, 20.0, 50.0), (255.0, 10.0, 10.0)];

const MOONLIGHT_COLORS: [(f32, f32, f32); 2] = [(255.0, 240.0, 245.0), (188.0, 143.0, 143.0)];

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
                brightness_increment: 0.1,
            })
        }

        for i in 0..nr_leds {
            target_led_vec.push(Led {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
                brightness_increment: 0.1,
            })
        }

        LedManager {
            nr_leds: nr_leds,
            current_theme: ColorTheme::NoLight,
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

    pub fn set_theme(&mut self, theme: ColorTheme) {
        self.current_theme = theme;
        match theme {
            ColorTheme::NoLight => self.set_all_colors(&NO_LIGHT_COLORS),
            ColorTheme::Sunrise => self.set_all_colors(&SUNRISE_COLORS),
            ColorTheme::Noon => self.set_all_colors(&NOON_COLORS),
            ColorTheme::Sunset => self.set_all_colors(&SUNSET_COLORS),
            ColorTheme::Moonlight => self.set_all_colors(&MOONLIGHT_COLORS),
        }
        self.update();
    }

    pub fn set_all_colors(&mut self, colors: &[(f32, f32, f32)]) {
        let mut index = 0;
        for target in self.target_leds.iter_mut() {
            let (red, green, blue) = colors[index];
            target.set_rgb(red, green, blue);
            index += 1;
            if index >= colors.len() {
                index = 0;
            }
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

    pub fn set_increment(&mut self, increment: f32) {
        for led in self.current_leds.iter_mut() {
            led.brightness_increment = increment;
        }
    }

    pub fn set_all_rgb(&mut self, red: f32, green: f32, blue: f32) {
        for target in self.target_leds.iter_mut() {
            target.set_rgb(red, green, blue);
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
                ValueComparison::Greater => {
                    led.set_brightness(led.alpha - led.brightness_increment)
                }
                ValueComparison::Lesser => led.set_brightness(led.alpha + led.brightness_increment),
                ValueComparison::Equal => {}
            }

            match compare_values(diff_red, 0.0) {
                ValueComparison::Greater => {
                    led.set_rgb(led.red - led.brightness_increment, led.green, led.blue)
                }
                ValueComparison::Lesser => {
                    led.set_rgb(led.red + led.brightness_increment, led.green, led.blue)
                }
                ValueComparison::Equal => {}
            }

            match compare_values(diff_green, 0.0) {
                ValueComparison::Greater => {
                    led.set_rgb(led.red, led.green - led.brightness_increment, led.blue)
                }
                ValueComparison::Lesser => {
                    led.set_rgb(led.red, led.green + led.brightness_increment, led.blue)
                }
                ValueComparison::Equal => {}
            }

            match compare_values(diff_blue, 0.0) {
                ValueComparison::Greater => {
                    led.set_rgb(led.red, led.green, led.blue - led.brightness_increment)
                }
                ValueComparison::Lesser => {
                    led.set_rgb(led.red, led.green, led.blue + led.brightness_increment)
                }
                ValueComparison::Equal => {}
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
            }
        }
        self.tx.start_blocking(&signal).unwrap();
    }
}

pub struct Led {
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
