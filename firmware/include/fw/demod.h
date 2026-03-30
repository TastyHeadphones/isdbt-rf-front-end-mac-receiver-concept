#ifndef FW_DEMOD_H
#define FW_DEMOD_H

#include <stdbool.h>

#include "fw/status.h"

typedef struct
{
  bool locked;
  float snr_db;
  float ber;
} fw_demod_metrics_t;

fw_status_t fw_demod_init(void);
fw_status_t fw_demod_get_metrics(fw_demod_metrics_t *metrics);

#endif
