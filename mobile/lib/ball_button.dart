import 'package:flutter/material.dart';
import 'package:blurrycontainer/blurrycontainer.dart';

enum ColorScheme {
  noLight,
  moonlight,
  sunrise,
  noon,
  sunset,
}

class BallButton extends StatefulWidget {
  const BallButton({Key? key, required this.currentTime}) : super(key: key);

  final DateTime currentTime;

  @override
  _BallButtonState createState() => _BallButtonState();
}

class _BallButtonState extends State<BallButton> with TickerProviderStateMixin {
  late AnimationController controller;
  late Animation<double> animation;
  late Color firstColor;
  late Color secondColor;
  bool isOn = true;
  ColorScheme colorScheme = ColorScheme.noLight;

  @override
  void initState() {
    super.initState();
    controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 10),
    );
    animation = CurvedAnimation(parent: controller, curve: Curves.easeIn);
    controller.repeat();
    updateColors();
  }

  void updateColors() {
    final now = widget.currentTime;
    final sunrise = DateTime(now.year, now.month, now.day, 6, 30, 0);
    final sunset = DateTime(now.year, now.month, now.day, 18, 0, 0);

    if (!isOn) {
      colorScheme = ColorScheme.noLight;
    } else if (now.isAfter(sunrise) &&
        now.isBefore(sunrise.add(const Duration(minutes: 30)))) {
      colorScheme = ColorScheme.sunrise;
    } else if (now.isAfter(sunset) &&
        now.isBefore(sunset.add(const Duration(minutes: 30)))) {
      colorScheme = ColorScheme.sunset;
    } else if (now.isAfter(sunrise.add(const Duration(minutes: 31))) &&
        now.isBefore(sunset)) {
      colorScheme = ColorScheme.noon;
    } else {
      colorScheme = ColorScheme.moonlight;
    }

    final colors = getColorSchemeColors(colorScheme);
    firstColor = colors[0];
    secondColor = colors[1];
  }

  void toggleOnOff() {
    setState(() {
      isOn = !isOn;
    });
    updateColors();
  }

  List<Color> getColorSchemeColors(ColorScheme scheme) {
    switch (scheme) {
      case ColorScheme.noLight:
        return [Colors.black45, Colors.blueGrey];
      case ColorScheme.moonlight:
        return [Colors.indigo, Colors.pink];
      case ColorScheme.sunrise:
        return [Colors.amber, Colors.pink];
      case ColorScheme.noon:
        return [Colors.lightBlue, Colors.yellow];
      case ColorScheme.sunset:
        return [Colors.amber, Colors.deepOrange];
    }
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      behavior: HitTestBehavior.translucent,
      onTap: toggleOnOff,
      child: Center(
        child: Stack(
          alignment: Alignment.center,
          children: [
            RotationTransition(
              turns: animation,
              child: Container(
                height: 200,
                width: 200,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  gradient: LinearGradient(
                    colors: [firstColor, secondColor],
                  ),
                ),
              ),
            ),
            const Positioned(
              child: BlurryContainer(
                blur: 12,
                width: 245,
                height: 245,
                elevation: 0,
                color: Colors.transparent,
                padding: EdgeInsets.all(10),
                borderRadius: BorderRadius.all(Radius.circular(20)),
                child: OverlayBall(),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class OverlayBall extends StatelessWidget {
  const OverlayBall({
    Key? key,
    this.size = const Size.square(170),
  }) : super(key: key);
  final Size size;
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Container(
      margin: const EdgeInsets.all(12.0),
      height: size.height,
      width: size.width,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        color: theme.colorScheme.background,
      ),
    );
  }
}
