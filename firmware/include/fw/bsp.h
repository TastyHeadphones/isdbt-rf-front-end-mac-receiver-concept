#ifndef FW_BSP_H
#define FW_BSP_H

#include <stdbool.h>

#include "fw/status.h"

typedef struct
{
  bool rails_ok;
  bool clocks_ok;
  bool reset_ok;
} fw_bsp_state_t;

fw_status_t fw_bsp_init(fw_bsp_state_t *state);

#endif
