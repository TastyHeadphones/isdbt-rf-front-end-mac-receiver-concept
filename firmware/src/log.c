#include "fw/log.h"

#include <stdio.h>
#include <string.h>

static fw_log_fn_t s_logger = NULL;
static void *s_logger_user = NULL;

void fw_log_set_hook(fw_log_fn_t fn, void *user)
{
  s_logger = fn;
  s_logger_user = user;
}

void fw_log_write(fw_log_level_t level, const char *fmt, ...)
{
  char buffer[256];
  va_list args;

  va_start(args, fmt);
  vsnprintf(buffer, sizeof(buffer), fmt, args);
  va_end(args);

  if (s_logger != NULL)
  {
    s_logger(level, buffer, s_logger_user);
    return;
  }

  const char *prefix = "INFO";
  switch (level)
  {
  case FW_LOG_DEBUG:
    prefix = "DEBUG";
    break;
  case FW_LOG_INFO:
    prefix = "INFO";
    break;
  case FW_LOG_WARN:
    prefix = "WARN";
    break;
  case FW_LOG_ERROR:
    prefix = "ERROR";
    break;
  default:
    break;
  }

  fprintf(stderr, "[fw][%s] %s\n", prefix, buffer);
}
