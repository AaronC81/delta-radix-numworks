// Adapts EADK functions with "simpler" type signatures, allowing them to be called from Rust
// without problems.
//
// For consistency, even functions which could probably be called from Rust without problems are
// bridged here.
//
// Targets the `eadk.h` distributed with `nwlink` 0.0.17.

#include <eadk.h>
#include <stdint.h>

// Keyboard and Events

uint64_t eadk_bridge__keyboard_scan() {
    return eadk_keyboard_scan();
}

// Display

void eadk_bridge__display_push_rect(uint16_t x, uint16_t y, uint16_t width, uint16_t height, const uint16_t* pixels) {
    eadk_display_push_rect((eadk_rect_t){ x, y, width, height }, pixels);
}

void eadk_bridge__display_push_rect_uniform(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t color) {
    eadk_display_push_rect_uniform((eadk_rect_t){ x, y, width, height }, (eadk_color_t){color});
}

void eadk_bridge__display_pull_rect(uint16_t x, uint16_t y, uint16_t width, uint16_t height, eadk_color_t * pixels) {
    eadk_display_push_rect((eadk_rect_t){ x, y, width, height }, pixels);
}

uint8_t eadk_bridge__display_wait_for_vblank() {
    return (uint8_t)eadk_display_wait_for_vblank();
}

void eadk_bridge__display_draw_string(const char *str, uint16_t x, uint16_t y, uint8_t large_font, uint16_t text_color, uint16_t bg_color) {
    eadk_display_draw_string(str, (eadk_point_t){x, y}, large_font, text_color, bg_color);
}

// Timing

void eadk_bridge__timing_usleep(uint32_t us) {
    eadk_timing_usleep(us);
}

void eadk_bridge__timing_msleep(uint32_t ms) {
    eadk_timing_msleep(ms);
}

uint64_t eadk_bridge__timing_millis() {
    return eadk_timing_millis();
}



