#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "stdint.h"

uint8_t *bdk_get_public_key(const uint8_t *data, uintptr_t len, uintptr_t *out_len);
