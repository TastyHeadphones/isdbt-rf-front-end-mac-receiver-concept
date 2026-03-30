#include "fw/bsp.h"
#include "fw/log.h"
#include <stddef.h>

fw_status_t fw_bsp_init(fw_bsp_state_t *state)
{
  if (state == NULL)
  {
    return FW_STATUS_INVALID_ARG;
  }

  state->rails_ok = true;
  state->clocks_ok = true;
  state->reset_ok = true;

  fw_log_write(FW_LOG_INFO, "BSP initialized: rails=%d clocks=%d reset=%d",
               state->rails_ok,
               state->clocks_ok,
               state->reset_ok);

  return FW_STATUS_OK;
}
