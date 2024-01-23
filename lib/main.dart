import 'package:flutter/material.dart';
import 'package:rust_state/src/rust/api/simple.dart';
import 'package:rust_state/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late int? count = null;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Column(
            children: [
              Text(count.toString()),
              TextButton(
                  onPressed: () async {
                    await newCounter().then((value) {
                      count = value;
                    });

                    setState(() {});
                  },
                  child: Text("init")),
              TextButton(
                  onPressed: () async {
                    await add().then((value) {
                      count = value;
                    });

                    setState(() {});
                  },
                  child: Text("add")),
              TextButton(
                  onPressed: () async {
                    await minus().then((value) {
                      count = value;
                    });
                    setState(() {});
                  },
                  child: Text("minus"))
            ],
          ),
        ),
      ),
    );
  }
}
