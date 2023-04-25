import 'package:flutter/material.dart';
import 'time_form.dart';

class ControlPage extends StatefulWidget {
  const ControlPage({super.key});

  @override
  State<ControlPage> createState() => _ControlPageState();
}

class _ControlPageState extends State<ControlPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: diuAppBar(),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: const [
            SizedBox(
              height: 20.0,
            ),
            TimeForm(),
          ],
        ),
      ),
    );
  }

  AppBar diuAppBar() {
    return AppBar(
      title: const Text("DIU"),
    );
  }
}
