#include "fw/status.h"

fw_status_t demod_driver_write_reg(unsigned int reg, unsigned int value)
{
  (void)reg;
  (void)value;
  return FW_STATUS_OK;
}
