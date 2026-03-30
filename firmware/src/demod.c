#include "fw/demod.h"
#include "fw/log.h"
#include <stddef.h>

fw_status_t fw_demod_init(void)
{
  fw_log_write(FW_LOG_INFO, "Demod abstraction initialized");
  return FW_STATUS_OK;
}

fw_status_t fw_demod_get_metrics(fw_demod_metrics_t *metrics)
{
  if (metrics == NULL)
  {
    return FW_STATUS_INVALID_ARG;
  }

  metrics->locked = true;
  metrics->snr_db = 29.5f;
  metrics->ber = 0.00001f;

  return FW_STATUS_OK;
}
