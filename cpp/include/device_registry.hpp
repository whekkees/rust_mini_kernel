#pragma once

#include <stdint.h>

enum DeviceType : uint32_t {
    DEVICE_BLOCK = 0,
    DEVICE_OTHER = 1,
};

struct DeviceInfo {
    uint32_t id;
    uint32_t category;
    const char* name;
};

extern "C" {
    bool device_register(uint32_t id, uint32_t category, const char* name);
}