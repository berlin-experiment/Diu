extern crate alloc;

use esp32_nimble::{enums::*, utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use esp_idf_sys::ble_gap_conn_desc;

pub fn initialize_ble_server<F>(light_msg_handler: F)
where
    F: FnMut(&[u8], &ble_gap_conn_desc) -> () + Send + std::marker::Sync + 'static,
{
    let device = BLEDevice::take();
    device
        .security()
        .set_auth(true, true, true)
        .set_passkey(123456)
        .set_io_cap(SecurityIOCap::DisplayOnly);

    let server = device.get_server();
    let service = server.create_service(BleUuid::Uuid16(0xABCD));

    server.on_connect(|_| {
        device.get_advertising().start().unwrap();
    });
    // A writable characteristic.
    let writable_characteristic = service.lock().create_characteristic(
        uuid128!("3c9a3f00-8ed3-4bdf-8a39-000000000001"),
        NimbleProperties::READ | NimbleProperties::WRITE,
    );
    writable_characteristic
        .lock()
        .on_read(move |_, _| {
            println!("Read from writable characteristic.");
        })
        .on_write(light_msg_handler);

    writable_characteristic
        .lock()
        .set_value("Talk to Me".as_bytes());

    // non-secure characteristics
    let non_secure_characteristic = service
        .lock()
        .create_characteristic(BleUuid::Uuid16(0x1234), NimbleProperties::READ);
    non_secure_characteristic
        .lock()
        .set_value("non_secure_characteristic".as_bytes());

    // secure characteristics
    let secure_characteristic = service.lock().create_characteristic(
        BleUuid::Uuid16(0x1235),
        NimbleProperties::READ | NimbleProperties::READ_ENC | NimbleProperties::READ_AUTHEN,
    );
    secure_characteristic
        .lock()
        .set_value("secure_characteristic".as_bytes());

    let ble_advertising = device.get_advertising();
    ble_advertising
        .name("Diu-Demo")
        .add_service_uuid(BleUuid::Uuid16(0xABCD))
        .start()
        .unwrap();
}
