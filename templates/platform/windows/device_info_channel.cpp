#include "device_info_channel.h"

#include <windows.h>

#include <flutter/method_channel.h>
#include <flutter/standard_method_codec.h>

static void HandleMethodCall(
    const flutter::MethodCall<flutter::EncodableValue>& call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (call.method_name() == "getDeviceInfo") {
    flutter::EncodableMap info;
    info[flutter::EncodableValue("platform")] =
        flutter::EncodableValue("Windows");
    info[flutter::EncodableValue("version")] =
        flutter::EncodableValue("10+");

    char hostname[256] = {};
    DWORD size = sizeof(hostname);
    GetComputerNameA(hostname, &size);
    info[flutter::EncodableValue("hostname")] =
        flutter::EncodableValue(std::string(hostname));

    SYSTEM_INFO sysinfo = {};
    GetSystemInfo(&sysinfo);
    info[flutter::EncodableValue("processors")] = flutter::EncodableValue(
        static_cast<int>(sysinfo.dwNumberOfProcessors));

    result->Success(flutter::EncodableValue(info));
  } else {
    result->NotImplemented();
  }
}

void RegisterDeviceInfoChannel(flutter::FlutterEngine* engine) {
  auto channel =
      std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
          engine->messenger(), "{{channel_prefix}}/device",
          &flutter::StandardMethodCodec::GetInstance());
  channel->SetMethodCallHandler(HandleMethodCall);
}
