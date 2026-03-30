#ifndef FW_LOG_H
#define FW_LOG_H

#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
  FW_LOG_DEBUG = 0,
  FW_LOG_INFO = 1,
  FW_LOG_WARN = 2,
  FW_LOG_ERROR = 3
} fw_log_level_t;

typedef void (*fw_log_fn_t)(fw_log_level_t level, const char *message, void *user);

void fw_log_set_hook(fw_log_fn_t fn, void *user);
void fw_log_write(fw_log_level_t level, const char *fmt, ...);

#ifdef __cplusplus
}
#endif

#endif
