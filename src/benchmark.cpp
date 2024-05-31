//
// Created by jschwender on 5/31/24.
//

#include <string>
#include "benchmark.h"
void EmptyFunc() {}

extern "C" void EmptyFuncV2() {}

int32_t add1(int32_t i) {
    return i + 1;
}

extern "C" int32_t add1_v2(int32_t i) {
    return i + 1;
}

uint32_t SumSlice(rust::Slice<const uint32_t> v) {
    uint32_t sum = 0;
    for (auto& n : v) {
        sum += n;
    }
    return sum;
}

//std::vector<uint32_t> CreateCxxVec(uint32_t len) {
//    std::vector<int> v;
//    for (uint32_t i = 0; i < len: i++ ) {
//        v[i] = i;
//    }
//    return v;
//}
//
//uint32_t SumCxxVector(std::vector<uint32_t> v) {
//    uint32_t sum = 0;
//    for (auto& n : v) {
//        sum += n;
//    }
//    return sum;
//}
