#include <assert.h>

#include "fw/control.h"
#include "fw/self_test.h"

int main(void)
{
  assert(fw_init_sequence() == FW_STATUS_OK);
  assert(fw_tune(569142857U, 6U) == FW_STATUS_OK);
  assert(fw_run_self_test() == FW_STATUS_OK);
  return 0;
}
