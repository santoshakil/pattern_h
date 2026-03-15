#include "device_info_channel.h"

#include <sys/utsname.h>
#include <unistd.h>

static void handle_method_call(FlMethodChannel* channel,
                               FlMethodCall* method_call,
                               gpointer user_data) {
  const gchar* method = fl_method_call_get_name(method_call);
  if (g_strcmp0(method, "getDeviceInfo") == 0) {
    g_autoptr(FlValue) result = fl_value_new_map();
    fl_value_set_string_take(result, "platform",
                             fl_value_new_string("Linux"));
    struct utsname uts;
    if (uname(&uts) == 0) {
      fl_value_set_string_take(result, "version",
                               fl_value_new_string(uts.release));
      fl_value_set_string_take(result, "hostname",
                               fl_value_new_string(uts.nodename));
    }
    fl_value_set_string_take(result, "processors",
                             fl_value_new_int(sysconf(_SC_NPROCESSORS_ONLN)));
    fl_method_call_respond_success(method_call, result, nullptr);
  } else {
    fl_method_call_respond_not_implemented(method_call, nullptr);
  }
}

void register_device_info_channel(FlView* view) {
  FlEngine* engine = fl_view_get_engine(view);
  g_autoptr(FlStandardMethodCodec) codec = fl_standard_method_codec_new();
  FlMethodChannel* channel = fl_method_channel_new(
      fl_engine_get_binary_messenger(engine), "{{channel_prefix}}/device",
      FL_METHOD_CODEC(codec));
  fl_method_channel_set_method_call_handler(channel, handle_method_call,
                                            nullptr, nullptr);
}
