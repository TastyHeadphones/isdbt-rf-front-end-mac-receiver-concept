#include "fw/control.h"

#include "fw/bridge.h"
#include "fw/bsp.h"
#include "fw/demod.h"
#include "fw/log.h"
#include "fw/tuner.h"

fw_status_t fw_init_sequence(void)
{
  fw_bsp_state_t bsp = {0};

  if (fw_bsp_init(&bsp) != FW_STATUS_OK)
  {
    return FW_STATUS_FAIL;
  }

  if (!bsp.rails_ok || !bsp.clocks_ok || !bsp.reset_ok)
  {
    return FW_STATUS_NOT_READY;
  }

  if (fw_bridge_init() != FW_STATUS_OK)
  {
    return FW_STATUS_FAIL;
  }

  if (fw_tuner_init() != FW_STATUS_OK)
  {
    return FW_STATUS_FAIL;
  }

  if (fw_demod_init() != FW_STATUS_OK)
  {
    return FW_STATUS_FAIL;
  }

  fw_log_write(FW_LOG_INFO, "Firmware init sequence complete");
  return FW_STATUS_OK;
}

fw_status_t fw_tune(uint32_t frequency_hz, uint8_t bandwidth_mhz)
{
  fw_tuner_params_t params = {
    .frequency_hz = frequency_hz,
    .bandwidth_mhz = bandwidth_mhz,
  };

  return fw_tuner_configure(&params);
}
