import 'package:hooks/hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    await const RustBuilder(
      assetName: 'package:native_bridge/native_bridge.dart',
      cratePath: '../../../rust',
      extraCargoBuildArgs: ['-p', 'app_core'],
    ).run(input: input, output: output);
  });
}
