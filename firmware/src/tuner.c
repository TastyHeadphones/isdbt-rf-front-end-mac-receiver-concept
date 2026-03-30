#include "fw/tuner.h"
#include "fw/log.h"
#include <stddef.h>

fw_status_t fw_tuner_init(void)
{
  fw_log_write(FW_LOG_INFO, "Tuner abstraction initialized");
  return FW_STATUS_OK;
}

fw_status_t fw_tuner_configure(const fw_tuner_params_t *params)
{
  if (params == NULL)
  {
    return FW_STATUS_INVALID_ARG;
  }

  fw_log_write(FW_LOG_INFO,
               "Tuner configured: frequency_hz=%u bandwidth_mhz=%u",
               params->frequency_hz,
               params->bandwidth_mhz);

  return FW_STATUS_OK;
}
