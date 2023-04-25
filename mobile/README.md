# Diu

Diu is a smart lighting system that emulates natural diurnal light cycles. It features a circular arrangement of 12 WS2812B LEDs that indicate the hours of the day using four distinct color schemes: sunrise, daylight, sunset, and moonlight. The system allows users to define their preferred sunrise and sunset times via a mobile app built with Flutter. The app communicates securely with the microcontroller using Bluetooth Low Energy (BLE), enabling users to remotely control the Diu system.

## Features

- Seamless emulation of natural diurnal light cycles
- Circular arrangement of 12 WS2812B LEDs indicating the hours of the day
- Four distinct color schemes: sunrise, daylight, sunset, and moonlight
- User-defined sunrise and sunset times
- Mobile app built with Flutter for easy configuration and remote control
- Secure and efficient communication via Bluetooth Low Energy (BLE)

## System Architecture

The Diu system comprises two main components:

1. **Mobile App (Flutter):** This component provides the user interface for setting sunrise and sunset times and managing the Diu system. It communicates with the microcontroller using BLE.
2. **Seeed Studio XIAO Microcontroller:** This component controls the brightness, color, and transition patterns of the LEDs based on user-defined settings and current time. It communicates with the mobile app using BLE and at a later stage, with AWS cloud services via WiFi.

## Security Measures

The Diu system uses several security measures to ensure secure and efficient communication between the mobile app, microcontroller, and AWS cloud services. These measures include secure BLE communication, secure WiFi communication, and secure AWS communication.

## Getting Started

Please refer to the project documentation and setup guides for detailed instructions on configuring and deploying the Diu system. The documentation covers the following topics:

- Setting up the hardware components (Seeed Studio XIAO, WS2812B LEDs, and necessary connections)
- Building and deploying the Flutter mobile app
- Establishing secure communication between the mobile app and the microcontroller
- Troubleshooting common issues and FAQs

## Contributing

We welcome contributions to the Diu! Please feel free to submit issues, feature requests, or pull requests on the project's GitHub repository. Make sure to follow the provided coding and documentation standards.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

## Acknowledgements

- The Diu team would like to thank all contributors and users for their valuable feedback and support.
- Special thanks to the open-source community for providing invaluable resources and inspiration.
