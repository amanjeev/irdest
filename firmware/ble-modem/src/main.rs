use esp32_nimble::{
    enums::{ConnMode, DiscMode},
    BLEAdvertisementData, BLEDevice,
};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys as _;

fn main() {
    esp_idf_svc::sys::link_patches();
    let ble_device = BLEDevice::take();
    let ble_advertiser = ble_device.get_advertising();
    let mut ad_data = BLEAdvertisementData::new();
    ad_data.name("ESP32 Peripheral");
    ble_advertiser.lock().set_data(&mut ad_data).unwrap();
    ble_advertiser.lock().advertisement_type(ConnMode::Non).disc_mode(DiscMode::Gen).scan_response(false);
    ble_advertiser.lock().start().unwrap();
    println!("Advertising started");
    loop {
        FreeRtos::delay_ms(10);
    }
}