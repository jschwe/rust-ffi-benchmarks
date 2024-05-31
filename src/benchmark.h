#pragma once

#include "rust/cxx.h"
#include <cstdint>
void EmptyFunc();

extern "C" void EmptyFuncV2();

int32_t add1(int32_t i);
extern "C" int32_t add1_v2(int32_t i);

uint32_t SumSlice(rust::Slice<const uint32_t> v);


// std::vector<uint32_t> CreateCxxVec(uint32_t len);
// uint32_t SumCxxVector(std::vector<uint32_t> v);


