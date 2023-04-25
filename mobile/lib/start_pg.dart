import 'dart:async';
import 'package:Diu/app_bar.dart';
import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:provider/provider.dart';
import 'ble.dart';

class StartPage extends StatefulWidget {
  const StartPage({super.key});

  @override
  State<StartPage> createState() => _StartPageState();
}

class _StartPageState extends State<StartPage> {
  @override
  Widget build(BuildContext context) {
    return Consumer<BleService>(builder: (context, ble, child) {
      void selectResult(ScanResult result) async {
        ble.connectToDevice(result).then((result) => null);
      }

      Widget createTileItem(ScanResult result) {
        return ListTile(
          onTap: () {
            selectResult(result);
          },
          title: Text(result.device.name),
          trailing: Text(
            result.rssi.toString(),
          ),
        );
      }

      return Scaffold(
        appBar: diuAppBar(),
        body: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 20.0),
          child: ListView(
            children: [
              const SizedBox(
                height: 30,
              ),
              const Text(
                "List of Available Devices",
                style: TextStyle(fontSize: 18.0),
              ),
              const SizedBox(
                height: 10,
              ),
              Text(
                (ble.isOn && ble.isAvailable)
                    ? "BLE is Active"
                    : "Please turn on BLE",
              ),
              ...ble.scanResults.map(createTileItem).toList()
            ],
          ),
        ),
      );
    });
  }
}
