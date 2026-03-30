#ifndef FW_TUNER_H
#define FW_TUNER_H

#include <stdint.h>

#include "fw/status.h"

typedef struct
{
  uint32_t frequency_hz;
  uint8_t bandwidth_mhz;
} fw_tuner_params_t;

fw_status_t fw_tuner_init(void);
fw_status_t fw_tuner_configure(const fw_tuner_params_t *params);

#endif
