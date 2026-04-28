#include "example4.h"
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
    u8 reserved;
    u8 attributes;
    u16 addr_h;
};

struct AnilloIDTDescriptor {
    u16 size;
    u32 addr;
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
    .addr_l = ((u32) AnilloISR16 & 0xF),
    .seg_sel = 0x8,
    .attributes = 0b10001110,
    .addr_h = ((u32) AnilloISR16 >> 16)
};
idt[17] = (struct AnilloGateDescriptor) {
    .addr_l = ((u32) AnilloISR17 & 0xF),
    .seg_sel = 0x8,
    .attributes = 0b10001110,
    .addr_h = ((u32) AnilloISR17 >> 16)
};


    static volatile struct AnilloIDTDescriptor idtd = {255, (u32) &idt};
    asm volatile (
        "lidtl %0"
        :
        : "r" (&idtd)
    );
}