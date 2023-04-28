# Hardware Details

**Author:** Stacey Kenny  
**Institution:** CODE University of Applied Sciences  
**Course:** SE_12 Internet of Things, Coordinator: Ulrich von Zadow  
**Semester:** Spring 2023

## 1. System Overview

The IoT system developed in this project is a smart lighting system that can be controlled remotely using a Flutter application via WiFi and Bluetooth connectivity. The system utilizes a Seeed Studio Xiao ESP32C3 microcontroller, individually addressable WS2812B LEDs, push buttons, and a potentiometer. The purpose of this IoT system is to create a light alarm that serves as an alternative to a standard sound alarm and as an artificial sun during the winter months when daylight is significantly reduced in certain regions. The system is designed to provide users with a customizable lighting solution that can be adjusted to their preferences using both manual controls and remote access.

### Components

- **Seeed Studio Xiao ESP32C3 microcontroller:** Manages connectivity, processes data, and controls the LEDs.
- **Individually Addressable WS2812B LEDs:** Create the lighting system with individual control using the microcontroller.
- **Push Buttons:** Allow manual interaction with the lighting system (on/off, color scheme, brightness).
- **Potentiometer:** Adjusts the brightness of the LEDs by turning the knob.
- **Flutter Application:** Remotely controls the lighting system via WiFi or Bluetooth connectivity.

**Note:** The microcontroller code is located in the "diu/hardware/" directory, while the Flutter application code is in the "diu/mobile/" directory.

## 2. Design Choices

### 2.1 Seeed Studio Xiao ESP32C3 Microcontroller

#### 2.1.1 Requirements

- Low power consumption
- WiFi and Bluetooth connectivity
- Sufficient processing power and memory
- Available documentation and community support
- Reasonable cost

#### 2.1.2 Options

| Option                    | Description                                                                                                                                              | Link | Cost   | Processing Power | Memory                        | Power Consumption | Connectivity       |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- | ------ | ---------------- | ----------------------------- | ----------------- | ------------------ |
| ESP32                     | Dual-core MCU with WiFi and Bluetooth connectivity                                                                                                       | Link | €17.59 | 240MHz           | 4MB, 8 MB, or 16 MB           | -                 | WiFi and Bluetooth |
| Seeed Studio Xiao ESP32C3 | Ultra-low power MCU with WiFi and Bluetooth, sufficient processing power and memory, available documentation and community support, and reasonable cost. | Link | €14.90 | 160MHz           | 400KB SRAM, 4MB onboard Flash | Low               | WiFi and Bluetooth |
| ESP32-WROOM-32            | Popular option for IoT projects due to its WiFi and Bluetooth connectivity, processing power, and available documentation and community support.         | Link | €10.49 | 240MHz           | 4 MB Flash, 520 KB SRAM       | High              | WiFi and Bluetooth |

#### 2.1.3 Discussion

The Seeed Studio Xiao ESP32C3 was chosen as the best option due to its low power consumption, WiFi and Bluetooth connectivity, processing power, memory, documentation, community support, and cost. Personal preference and prior experience also influenced the decision.

### 2.2 Individually Addressable WS2812B LEDs

#### 2.2.1 Requirements

- Ability to be individually addressed and controlled
- Compatibility with the microcontroller

#### 2.2.2 Options

| Option | Description                                      | Link | Price              | Individually Addressable | Compatibility with microcontroller |
| ------ | ------------------------------------------------ | ---- | ------------------ | ------------------------ | ---------------------------------- |
| WS2812 | Individually addressable LED with RGB and W LEDs | Link | €9.49 (€0.38/each) | Yes                      | Compatible                         |
| APA102 | Individually addressable LED with RGB LEDs       | Link | €7.47              | Yes                      | Compatible                         |
| WS2813 | Individually addressable LED with RGB LEDs       | Link | €24.99(1m)         | Yes                      | Compatible                         |

#### 2.2.3 Discussion

Individually addressable WS2812B LEDs were chosen for their ability to be individually addressed and controlled, high refresh rate, and relatively low cost. They allow for complex animations and patterns and ensure smooth transitions between colors and animations. However, they have a limited color range compared to other types of LEDs and may require additional hardware, such as a level shifter, for optimal performance.

## 3. Summary

This project involves the development of a smart lighting system that can be remotely controlled using a Flutter application through WiFi and Bluetooth connectivity. The IoT system is designed as a light alarm to replace traditional sound alarms and provide an artificial sun during the winter months when daylight is scarce in certain regions.

The main components of the system include a Seeed Studio Xiao ESP32C3 microcontroller, individually addressable WS2812B LEDs, push buttons, and a potentiometer. The system is designed to offer customizable lighting solutions, allowing users to adjust their preferences using both manual controls and remote access.

The Seeed Studio Xiao ESP32C3 microcontroller was chosen for its low power consumption, WiFi and Bluetooth connectivity, sufficient processing power and memory, available documentation and community support, and reasonable cost. Individually addressable WS2812B LEDs were selected for their ability to be individually addressed and controlled, and compatibility with the microcontroller. The microcontroller code can be found in the "diu/hardware/" directory, and the Flutter application code is located in the "diu/mobile/" directory.
