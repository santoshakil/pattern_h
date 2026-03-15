#include "channel_registry.h"

#include "device_info_channel.h"

void register_channels(FlView* view) {
  register_device_info_channel(view);
}
