#include "fw/self_test.h"
#include "fw/demod.h"

fw_status_t fw_run_self_test(void)
{
  fw_demod_metrics_t metrics = {0};

  if (fw_demod_get_metrics(&metrics) != FW_STATUS_OK)
  {
    return FW_STATUS_FAIL;
  }

  return metrics.locked ? FW_STATUS_OK : FW_STATUS_NOT_READY;
}
