# Diu

## Features

* Seamless emulation of natural diurnal light cycles
* Circular arrangement of 12 WS2812B LEDs indicating the hours of the day
* Four distinct color schemes: sunrise, daylight, sunset, and moonlight
* User-defined sunrise and sunset times
* Mobile app built with Flutter for easy configuration and remote control
* Secure and efficient communication via Bluetooth Low Energy (BLE), WiFi, and AWS services

## System Architecture

1. **Mobile App (Flutter):** Provides the user interface for setting sunrise and sunset times and managing the Diu system. Communicates with the microcontroller via BLE.
2. **Seeed Studio XIAO Microcontroller:** Controls the brightness, color, and transition patterns of the LEDs based on user-defined settings and current time. Communicates with the mobile app using BLE and with AWS cloud via WiFi.
3. **AWS Cloud Infrastructure:** Handles secure communication, data storage, and remote control capabilities using services like API Gateway, Lambda Functions, Amazon S3, DynamoDB, AWS IoT Core, IoT Device Shadow, Cognito, and Amazon SNS.

## Security Measures

## Getting Started

Please refer to the project documentation and setup guides for detailed instructions on configuring and deploying the Diu system. The documentation covers the following topics:

* Setting up the hardware components (Seeed Studio XIAO, WS2812B LEDs, and necessary connections)
* Installing and configuring the required AWS services
* Building and deploying the Flutter mobile app
* Establishing secure communication between the mobile app, microcontroller, and AWS cloud services
* Troubleshooting common issues and FAQs

## Contributing

We welcome contributions to the Diu! Please feel free to submit issues, feature requests, or pull requests on the project's GitHub repository. Make sure to follow the provided coding and documentation standards.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

## Acknowledgements

* The Diu team would like to thank all contributors and users for their valuable feedback and support.
* Special thanks to the open-source community for providing invaluable resources and inspiration.
