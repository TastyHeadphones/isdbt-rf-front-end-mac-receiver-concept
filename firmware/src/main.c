#include <stdio.h>

#include "fw/control.h"
#include "fw/self_test.h"

int main(void)
{
  if (fw_init_sequence() != FW_STATUS_OK)
  {
    fprintf(stderr, "firmware init failed\n");
    return 1;
  }

  if (fw_tune(569142857U, 6U) != FW_STATUS_OK)
  {
    fprintf(stderr, "tune failed\n");
    return 1;
  }

  if (fw_run_self_test() != FW_STATUS_OK)
  {
    fprintf(stderr, "self-test failed\n");
    return 1;
  }

  fprintf(stdout, "firmware demo completed\n");
  return 0;
}
