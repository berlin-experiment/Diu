import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'app_bar.dart';
import 'ball_button.dart';
import 'ble.dart';
import 'time_form.dart';

class ControlPage extends StatefulWidget {
  const ControlPage({super.key});

  @override
  State<ControlPage> createState() => _ControlPageState();
}

class _ControlPageState extends State<ControlPage> {
  final DateTime currentTime = DateTime.now();
  @override
  Widget build(BuildContext context) {
    return Consumer<BleService>(builder: (context, ble, child) {
      if (!ble.isConnected) {
        Navigator.pop(context);
      }
      final now = DateTime.now();

      Future<void> toggleOnOff(bool value) async {
        ble.updateLeds(value ? "ON" : "OFF");
      }

      return Scaffold(
        appBar: diuAppBar(),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const SizedBox(
                height: 20.0,
              ),
              BallButton(
                currentTime: currentTime,
                toggleOnOff: toggleOnOff,
              ),
              TimeForm(),
            ],
          ),
        ),
      );
    });
  }
}
