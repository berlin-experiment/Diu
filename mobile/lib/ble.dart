import 'dart:convert';
import 'dart:developer';

import 'package:location/location.dart';

import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';

// Define the class that extends ChangeNotifier
class BleService extends ChangeNotifier {
  BleService() {
    init();
  }

  Future<void> init() async {
    isOn = await _flutterBlue.isOn;
    isAvailable = await _flutterBlue.isAvailable;

    notifyListeners();

    location = Location();

    bool _serviceEnabled;
    PermissionStatus _permissionGranted;
    LocationData _locationData;

    _serviceEnabled = await location.serviceEnabled();
    if (!_serviceEnabled) {
      _serviceEnabled = await location.requestService();
      if (!_serviceEnabled) {
        return;
      }
    }

    _permissionGranted = await location.hasPermission();
    if (_permissionGranted == PermissionStatus.denied) {
      _permissionGranted = await location.requestPermission();
      if (_permissionGranted != PermissionStatus.granted) {
        return;
      }
    }

    _locationData = await location.getLocation();

    // Start scanning
    if (!isOn) {
      _flutterBlue.turnOn();
    }

    startScan();
  }

  void startScan() {
    stopScan();
    _scanResults = [];
    // Start scanning
    if (isOn && isAvailable) {
      _flutterBlue.startScan(timeout: Duration(seconds: 6));
    }
    // Listen to scan results
    _flutterBlue.scanResults.listen((results) {
      log(results.length.toString());

      results.removeWhere((element) => element.device.name.isEmpty);
      results.retainWhere((element) =>
          element.device.name.toString().toLowerCase().contains("diu"));
      // do something with scan results
      _scanResults = results;
      notifyListeners();
    });
  }

  void stopScan() {
    _flutterBlue.stopScan();
  }

  Future<void> connectToDevice(ScanResult result) async {
    await disconnect();
    _currentDevice = result;
    _currentServices = [];
    _currentCharacteristics = [];
    _hasLedCharacteristic = false;
    await _currentDevice.device.connect();
    isConnected = true;
    _currentServices = await _currentDevice.device.discoverServices();
    _currentServices.forEach((element) => _currentCharacteristics = [
          ..._currentCharacteristics,
          ...element.characteristics
        ]);

    _currentCharacteristics.forEach((element) {
      log(element.uuid.toString());
      if (element.uuid.toString() == "3c9a3f00-8ed3-4bdf-8a39-000000000001") {
        _ledCharacteristic = element;
        _hasLedCharacteristic = true;
      }
    });
    notifyListeners();

    if (!_hasLedCharacteristic) {
      disconnect();
    }
    notifyListeners();
  }

  Future<void> disconnect() async {
    _flutterBlue.connectedDevices.asStream().forEach((element) {
      for (var element in element) {
        element.disconnect();
      }
    });
    if (isConnected) {
      await _currentDevice.device.disconnect();
      isConnected = false;
      notifyListeners();
    }
  }

  Future<void> updateLeds(String message) async {
    if (isConnected && _hasLedCharacteristic) {
      await _ledCharacteristic.write(utf8.encode(message));
    }
  }

  late bool isOn = false;
  late bool isAvailable = false;
  late bool isConnected = false;
  final FlutterBluePlus _flutterBlue = FlutterBluePlus.instance;
  List<ScanResult> _scanResults = [];
  List<ScanResult> get scanResults => _scanResults;
  late ScanResult _currentDevice;
  late List<BluetoothService> _currentServices;
  late List<BluetoothCharacteristic> _currentCharacteristics = [];
  late BluetoothCharacteristic _ledCharacteristic;
  bool _hasLedCharacteristic = false;
  late Location location;
}
