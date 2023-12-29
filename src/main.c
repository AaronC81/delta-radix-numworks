#include <eadk.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

const char eadk_app_name[] __attribute__((section(".rodata.eadk_app_name"))) = "App";
const uint32_t eadk_api_level  __attribute__((section(".rodata.eadk_api_level"))) = 0;

// Defined by Rust.
void rs_main(void);

int main(int argc, char * argv[]) {
  rs_main();
  while (1) {
    eadk_keyboard_scan();
  }
}
