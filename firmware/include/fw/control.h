#ifndef FW_CONTROL_H
#define FW_CONTROL_H

#include <stdint.h>

#include "fw/status.h"

fw_status_t fw_init_sequence(void);
fw_status_t fw_tune(uint32_t frequency_hz, uint8_t bandwidth_mhz);

#endif
