<div align="center">
  <img src="https://img.icons8.com/office/512/maintenance.png" alt="FAQ Icon from Icons8" height="80">
  <h1>Hardware Setup Guide</h1>
  <p>How to connect the SEEED Studio XIAO ESP32C3 to 12 WS2812B LEDs in series and set up the antenna, light diffuser, and LED arrangement.</p>
</div>

1. [Hardware Setup](#hardware-setup)
   1. [Gather the Required Components](#1-gather-the-required-components)
   2. [Arrange the WS2812B LEDs](#2-arrange-the-ws2812b-leds)
   3. [Connect the WS2812B LEDs](#3-connect-the-ws2812b-leds)
   4. [Connect the LEDs in Series](#4-connect-the-leds-in-series)
   5. [Connect the Antenna (if necessary)](#5-connect-the-antenna)
   6. [Apply the Light Diffuser](#6-apply-the-light-diffuser)
   7. [Power the Setup](#7-power-the-setup)
2. [Test and configure the system](#test-and-configure-the-system)

---

# Hardware Setup

## 1. Gather the required components:

- SEEED Studio XIAO ESP32C3
- 12 WS2812B LEDs
- Jumper wires
- Breadboard or custom PCB (optional)
- 5V power supply
- Light diffuser material (e.g., white acrylic sheet or frosted glass)
- Antenna (if not integrated into the ESP32C3)
- Mobile device with the Flutter app for configuration

## 2. Arrange the WS2812B LEDs:

- Choose your preferred layout: circular or linear arc.
- Place the LEDs in the desired arrangement, making sure the input and output pins are properly aligned.

## 3. Connect the WS2812B LEDs:

- Connect the DIN (Data Input) pin of the first LED to the D0 (GPIO2) pin of the XIAO ESP32C3 using a jumper wire.
- Connect the GND (Ground) pins of all the LEDs to the GND pin of the XIAO ESP32C3.
- Connect the 5V pins of all the LEDs to the 5V power supply.

<div align="center">
  <img src="https://files.seeedstudio.com/wiki/XIAO_WiFi/pin_map-2.png" alt="Pinout Diagram from https://seeeddstudio.com/XIAO_ESP32C3_Getting_Started/" height="200">
  <p>Pinout diagram</p>
</div>

## 4. Connect the LEDs in series:

- Connect the DOUT (Data Output) pin of the first LED to the DIN pin of the second LED.
- Repeat this process for all subsequent LEDs until they are all connected in series.

## 5. Connect the antenna (if necessary):

- If your XIAO ESP32C3 module does not have an integrated antenna, connect the appropriate antenna to the antenna pin or connector on the ESP32C3.

<div align="center">
  <img src="https://files.seeedstudio.com/wiki/XIAO_WiFi/front-label-3.png" alt="Component overview from https://seeeddstudio.com/XIAO_ESP32C3_Getting_Started/" height="300">
  <p>Component overview</p>
</div>

## 6. Apply the light diffuser:

- Cut the light diffuser material to the appropriate size and shape to cover the WS2812B LEDs.
- Place the diffuser over the LEDs, ensuring it doesn't obstruct the signal, GND, or 5V connections.

## 7. Power the setup:

- Connect the 5V power supply to the XIAO ESP32C3 and ensure the WS2812B LEDs are receiving power.

<div align="center">
  <img src="https://files.seeedstudio.com/wiki/XIAO_WiFi/cable-connect.png" alt="Component overview from https://seeeddstudio.com/XIAO_ESP32C3_Getting_Started/" height="300">
  <p>Connect the 5V power supply</p>
</div>

# Test and configure the system:

- Upload the required code to the XIAO ESP32C3 to control the WS2812B LEDs, set the color schemes, and manage BLE communication.
- Install the Flutter mobile app on your mobile device.
- Test the BLE communication between the XIAO ESP32C3 and the mobile app to ensure proper configuration and control.

---

### Your Diu project hardware setup should now be complete. Enjoy your seamless emulation of natural diurnal light cycles with user-defined sunrise and sunset times, configurable through the mobile app.

---
