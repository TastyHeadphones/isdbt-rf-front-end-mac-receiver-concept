#include "fw/status.h"

fw_status_t bridge_driver_tx(const unsigned char *data, unsigned int len)
{
  (void)data;
  (void)len;
  return FW_STATUS_OK;
}
