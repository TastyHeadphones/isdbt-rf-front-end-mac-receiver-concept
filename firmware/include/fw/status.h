#ifndef FW_STATUS_H
#define FW_STATUS_H

typedef enum
{
  FW_STATUS_OK = 0,
  FW_STATUS_INVALID_ARG = -1,
  FW_STATUS_IO = -2,
  FW_STATUS_NOT_READY = -3,
  FW_STATUS_FAIL = -4
} fw_status_t;

#endif
