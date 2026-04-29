#include "test.h"
#include "_types.h"

typedef __uint8_t u8;
typedef __uint16_t u16;
typedef __uint32_t u32;
typedef __uint64_t u64;
typedef __int8_t i8;
typedef __int16_t i16;
typedef __int32_t i32;
typedef __int64_t i64;

struct __attribute__((packed)) AnilloGateDescriptor {
    u16 addr_l;
    u16 seg_sel;
    u8 ist;
    u8 attributes;
    u16 addr_m;
    u32 addr_h;
    u32 reserved;
};

struct AnilloIDTDescriptor {
    u16 size;
    u64 addr;
};

extern void GenericPicHandler(u8 irq);

void AnilloISR16() __attribute__((naked));
void AnilloISR16() {
    GenericPicHandler(16);
}
void AnilloISR17() __attribute__((naked));
void AnilloISR17() {
    GenericPicHandler(17);
}

void AnilloISRRegister() {
    static volatile struct AnilloGateDescriptor idt[256];
    idt[16] = (struct AnilloGateDescriptor) {
    .addr_l = ((u64) AnilloISR16 & 0xF),
    .seg_sel = 0x8,
    .ist = 0x0,
    .attributes = 0b10001110,
    .addr_m = ((u64) AnilloISR16 >> 16) & 0xF,
    .addr_h = ((u64) AnilloISR16 >> 32)
};
idt[17] = (struct AnilloGateDescriptor) {
    .addr_l = ((u64) AnilloISR17 & 0xF),
    .seg_sel = 0x8,
    .ist = 0x0,
    .attributes = 0b10001110,
    .addr_m = ((u64) AnilloISR17 >> 16) & 0xF,
    .addr_h = ((u64) AnilloISR17 >> 32)
};


    static volatile struct AnilloIDTDescriptor idtd = {255, &idt};
    asm volatile (
        "lidtl %0"
        :
        : "r" (&idtd)
    );
}