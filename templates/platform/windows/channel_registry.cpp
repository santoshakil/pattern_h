#include "channel_registry.h"

#include "device_info_channel.h"

void RegisterChannels(flutter::FlutterEngine* engine) {
  RegisterDeviceInfoChannel(engine);
}
