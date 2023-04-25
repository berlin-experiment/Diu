import 'package:flutter/material.dart';

class TimeForm extends StatefulWidget {
  const TimeForm({super.key});

  @override
  State<TimeForm> createState() => _TimeFormState();
}

class _TimeFormState extends State<TimeForm> {
  final _formKey = GlobalKey<FormState>();
  TimeOfDay sunriseTime =
      const TimeOfDay(hour: 6, minute: 0); // Default sunrise time
  TimeOfDay sunsetTime =
      const TimeOfDay(hour: 18, minute: 0); // Default sunset time

  Future<void> _selectSunriseTime(BuildContext context) async {
    final TimeOfDay? picked = await showTimePicker(
      context: context,
      initialTime: sunriseTime,
    );
    if (picked != null && picked != sunriseTime) {
      setState(() {
        sunriseTime = picked;
      });
    }
  }

  Future<void> _selectSunsetTime(BuildContext context) async {
    final TimeOfDay? picked = await showTimePicker(
      context: context,
      initialTime: sunsetTime,
    );
    if (picked != null && picked != sunsetTime) {
      setState(() {
        sunsetTime = picked;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 20.0),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            TextButton(
              onPressed: () => _selectSunriseTime(context),
              child:
                  Text('Select Sunrise Time (${sunriseTime.format(context)})'),
            ),
            const SizedBox(height: 20.0),
            TextButton(
              onPressed: () => _selectSunsetTime(context),
              child: Text('Select Sunset Time (${sunsetTime.format(context)})'),
            ),
            const SizedBox(height: 20.0),
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 16.0),
              child: ElevatedButton(
                onPressed: () {
                  if (_formKey.currentState!.validate()) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Processing Data')),
                    );
                  }
                },
                child: const Text('Submit'),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
