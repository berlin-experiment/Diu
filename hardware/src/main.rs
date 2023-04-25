use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;

mod bluetooth;
mod led_manager;

use bluetooth::initialize_ble_server;
use led_manager::LedManager;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let led_manager_pointer = std::sync::Arc::new(std::sync::Mutex::new(LedManager::new(2)));

    let led_manager_clone = std::sync::Arc::clone(&led_manager_pointer);
    let mut led_manager = led_manager_pointer.lock().unwrap();

    led_manager.set_increment(0.01);
    led_manager.set_all_rgb(150.0, 0.0, 255.0);
    drop(led_manager);

    let light_msg_handler = move |value: &[u8], _param: &esp_idf_sys::ble_gap_conn_desc| {
        let mut led_manager = led_manager_clone.lock().unwrap();
        let string = std::str::from_utf8(value).unwrap();
        if string.to_uppercase().contains("ON") {
            led_manager.turn_on();
            println!("{}", "Turning on");
        }
        if string.to_uppercase().contains("OFF") {
            led_manager.turn_off();
            println!("{}", "Turning off");
        }
        println!("{}", string);
        drop(led_manager);
    };

    initialize_ble_server(light_msg_handler);

    loop {
        let mut led_manager = led_manager_pointer.lock().unwrap();
        led_manager.update();
        drop(led_manager);
        FreeRtos::delay_ms(10);
    }
}
