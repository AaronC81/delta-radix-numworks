Q ?= @
CC = arm-none-eabi-gcc
BUILD_DIR = output
RS_BUILD_DIR = rs/target/thumbv7em-none-eabihf/debug
NWLINK = npx --yes -- nwlink@0.0.16
LINK_GC = 1
LTO = 1

define object_for
$(addprefix $(BUILD_DIR)/,$(addsuffix .o,$(basename $(1))))
endef

src = $(addprefix src/,\
  main.c \
  eadk_bridge.c \
)

CFLAGS = -std=c99
CFLAGS += $(shell $(NWLINK) eadk-cflags)
CFLAGS += -Os -Wall
CFLAGS += -ggdb
LDFLAGS = -Wl,--relocatable
LDFLAGS += -nostartfiles
LDFLAGS += --specs=nano.specs
# LDFLAGS += --specs=nosys.specs # Alternatively, use full-fledged newlib

ifeq ($(LINK_GC),1)
CFLAGS += -fdata-sections -ffunction-sections
LDFLAGS += -Wl,-e,main -Wl,-u,eadk_app_name -Wl,-u,eadk_app_icon -Wl,-u,eadk_api_level
LDFLAGS += -Wl,--gc-sections
endif

ifeq ($(LTO),1)
CFLAGS += -flto -fno-fat-lto-objects
CFLAGS += -fwhole-program
CFLAGS += -fvisibility=internal
LDFLAGS += -flinker-output=nolto-rel
endif

.PHONY: build
build: $(BUILD_DIR)/app.nwa

.PHONY: check
check: $(BUILD_DIR)/app.bin

.PHONY: run
run: $(BUILD_DIR)/app.nwa src/input.txt
	@echo "INSTALL $<"
	$(Q) $(NWLINK) install-nwa --external-data src/input.txt $<

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.nwa src/input.txt
	@echo "BIN     $@"
	$(Q) $(NWLINK) nwa-bin --external-data src/input.txt $< $@

$(BUILD_DIR)/%.elf: $(BUILD_DIR)/%.nwa src/input.txt
	@echo "ELF     $@"
	$(Q) $(NWLINK) nwa-elf --external-data src/input.txt $< $@

$(BUILD_DIR)/app.nwa: $(call object_for,$(src)) $(BUILD_DIR)/icon.o $(RS_BUILD_DIR)/librs.a
	@echo "LD      $@"
	$(Q) $(CC) $(CFLAGS) $(LDFLAGS) $^ $(RS_BUILD_DIR)/librs.a -o $@

$(addprefix $(BUILD_DIR)/,%.o): %.c | $(BUILD_DIR)
	@echo "CC      $^"
	$(Q) $(CC) $(CFLAGS) -c $^ -o $@

$(BUILD_DIR)/icon.o: src/icon.png
	@echo "ICON    $<"
	$(Q) $(NWLINK) png-icon-o $< $@

# For a Cargo project, figuring out "up-to-date-ness" is hard. Just rebuild every time; this doesn't
# take long.
.PHONY: $(RS_BUILD_DIR)/librs.a
$(RS_BUILD_DIR)/librs.a:
	@echo "CARGO   $@"
	$(Q) (cd rs && cargo build)

.PRECIOUS: $(BUILD_DIR)
$(BUILD_DIR):
	$(Q) mkdir -p $@/src

.PHONY: clean
clean:
	@echo "CLEAN"
	$(Q) rm -rf $(BUILD_DIR)
