import 'dart:io';
import 'dart:convert';

// read environment variable ROOT_DIRECTORY
final ROOT_DIRECTORY = Platform.environment['ROOT_DIRECTORY'];

void main(List<String> args) async {
  if (args.length != 1) {
    print('You must pass exactly one argument.');
    return;
  }

  String mode = args[0];
  print("Generating config file for $mode mode.");

  if (mode != 'playground' && mode != 'local' && mode != 'mainnet') {
    print('Invalid argument. Must be one of: playground, local, mainnet');
    return;
  }

  var filePath = '$ROOT_DIRECTORY/.dfx/$mode/canister_ids.json';
  print('filePath: $filePath');

  var file = File(filePath);
  print('file: $file');

  if (mode == 'mainnet') {
    var backendResult = await Process.run(
        'dfx', ['canister', 'id', 'mahjong_icp_backend', '--network=ic']);
    var frontendResult = await Process.run(
        'dfx', ['canister', 'id', 'mahjong_icp_frontend', '--network=ic']);
    await writeConfigFiles(
        frontendResult.stdout.trim(), backendResult.stdout.trim(), mode);
  } else if (await file.exists()) {
    print('File $file exists.');

    var content = await file.readAsString();
    print('content: $content');

    var jsonContent = jsonDecode(content);
    print('jsonContent: $jsonContent');

    var backendCanisterId = jsonContent['mahjong_icp_backend'][mode];
    print('backendCanisterId: $backendCanisterId');

    var frontend_canister_id = jsonContent['mahjong_icp_frontend'][mode];
    print('frontend_canister_id: $frontend_canister_id');

    //to file web_front_end.sh print https://<frontend_canister_id>.icp0.io/

    await writeConfigFiles(frontend_canister_id, backendCanisterId, mode);
  } else {
    print('File $filePath does not exist.');
  }
}

Future<void> writeConfigFiles(
    String frontend_canister_id, String backendCanisterId, String mode) async {
  var outputFile = File('$ROOT_DIRECTORY/web_front_end.sh');
  await outputFile.writeAsString('''
export FRONTEND_CANISTER_ID=$frontend_canister_id
  ''');

  outputFile = File(
      '$ROOT_DIRECTORY/src/mahjong_icp_frontend/lib/engine/db_implementations/ICP/config.dart');
  await outputFile.writeAsString('''
const backendCanisterId = '$backendCanisterId';
enum Mode {    playground,    local,    mainnet  }
Mode mode = Mode.$mode;

  ''');

  print('File generated successfully.');
}
