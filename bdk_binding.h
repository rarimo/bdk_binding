#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "stdint.h"

uint8_t *bdk_get_public_key(const uint8_t *data,
                            uintptr_t len,
                            uint8_t network_id,
                            uintptr_t *out_len);

const char *bdk_get_address(const uint8_t *data, uintptr_t len, uint8_t network_id);

uint8_t *bdk_alloc(uintptr_t len);

void bdk_dealloc(uint8_t *ptr, uintptr_t len);
