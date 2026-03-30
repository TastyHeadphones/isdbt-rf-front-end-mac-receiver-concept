#include "fw/bridge.h"
#include "fw/log.h"

fw_status_t fw_bridge_init(void)
{
  fw_log_write(FW_LOG_INFO, "Bridge abstraction initialized");
  return FW_STATUS_OK;
}

fw_status_t fw_bridge_poll(void)
{
  return FW_STATUS_OK;
}
