<div align="center">
  <img src="https://img.icons8.com/external-flaticons-lineal-color-flat-icons/512/external-faq-web-store-flaticons-lineal-color-flat-icons.png" alt="FAQ Icon from Icons8" height="80">
  <h1>Frequently Asked Questions</h1>
</div>

# Table of Contents

1. [What is Diu?](#1-what-is-diu)
2. [How do I set up the hardware components for Diu?](#2-how-do-i-set-up-the-hardware-components-for-diu)
3. [How do I configure the required AWS services for Diu?](#3-how-do-i-configure-the-required-aws-services-for-diu)
4. [How do I build and deploy the Flutter mobile app for Diu?](#4-how-do-i-build-and-deploy-the-flutter-mobile-app-for-diu)
5. [Can I change the sunrise and sunset times for Diu?](#5-can-i-change-the-sunrise-and-sunset-times-for-diu)
6. [What are the available color schemes in Diu?](#6-what-are-the-available-color-schemes-in-diu)
7. [How does Diu communicate with the mobile app and AWS cloud services?](#7-how-does-diu-communicate-with-the-mobile-app-and-aws-cloud-services)
8. [What security measures are implemented in Diu?](#8-what-security-measures-are-implemented-in-diu)
9. [How can I contribute to the Diu project?](#9-how-can-i-contribute-to-the-diu-project)
10. [What is the license for the Diu project?](#10-what-is-the-license-for-the-diu-project)

---

## 1. What is Diu?

Diu is a smart lighting system that emulates natural diurnal light cycles using a circular arrangement of 12 WS2812B LEDs. The system offers four distinct color schemes (sunrise, daylight, sunset, and moonlight) and allows users to define sunrise and sunset times. It can be configured and controlled via a mobile app built with Flutter and communicates securely and efficiently using Bluetooth Low Energy (BLE), WiFi, and AWS services.

## 2. How do I set up the hardware components for Diu?

Refer to the project documentation for detailed instructions on setting up the hardware components, including the Seeed Studio XIAO microcontroller, WS2812B LEDs, and necessary connections.

## 3. How do I configure the required AWS services for Diu?

The project documentation provides step-by-step instructions for installing and configuring the required AWS services, such as API Gateway, Lambda Functions, Amazon S3, DynamoDB, AWS IoT Core, IoT Device Shadow, Cognito, and Amazon SNS.

## 4. How do I build and deploy the Flutter mobile app for Diu?

Follow the instructions in the project documentation to set up the development environment, build, and deploy the Flutter mobile app for Diu.

## 5. Can I change the sunrise and sunset times for Diu?

Yes, you can define your preferred sunrise and sunset times using the Diu mobile app. The system will adjust the color transitions and brightness levels accordingly.

## 6. What are the available color schemes in Diu?

Diu offers four distinct color schemes: sunrise, daylight, sunset, and moonlight. These schemes are designed to emulate the natural diurnal light cycles and provide a comfortable and dynamic lighting experience.

## 7. How does Diu communicate with the mobile app and AWS cloud services?

Diu uses Bluetooth Low Energy (BLE) for communication between the mobile app and the microcontroller. For communication between the microcontroller and AWS cloud services, Diu uses WiFi.

## 8. What security measures are implemented in Diu?

Diu incorporates several security measures, including secure communication protocols, data storage, and remote control capabilities, using AWS services like API Gateway, Lambda Functions, Amazon S3, DynamoDB, AWS IoT Core, IoT Device Shadow, Cognito, and Amazon SNS.

## 9. How can I contribute to the Diu project?

We welcome contributions! Please feel free to submit issues, feature requests, or pull requests on the project's GitHub repository. Be sure to follow the provided coding and documentation standards.

## 10. What is the license for the Diu project?

Diu is licensed under the MIT License. For more details, see the LICENSE.md file.

---
