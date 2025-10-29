MEMORY
{
  /* ESP32-C3 Memory Layout */
  /* Note: ESP32-C3 uses RISC-V architecture, not ARM */
  
  /* Instruction RAM (IRAM) */
  IRAM : ORIGIN = 0x40380000, LENGTH = 400K
  
  /* Data RAM (DRAM) - usable SRAM */
  DRAM : ORIGIN = 0x3FC80000, LENGTH = 400K
  
  /* ROM (bootloader and system) */
  /* ROM : ORIGIN = 0x42000000, LENGTH = 384K */
  
  /* External Flash (via cache) */
  /* FLASH : ORIGIN = 0x42000000, LENGTH = 4M */
  
  /* RTC Fast Memory */
  RTC_FAST : ORIGIN = 0x50000000, LENGTH = 8K
  
  /* RTC Slow Memory */
  RTC_SLOW : ORIGIN = 0x50001000, LENGTH = 8K
}

/* Stack pointer initial value */
_stack_start = ORIGIN(DRAM) + LENGTH(DRAM);

SECTIONS
{
  .text : ALIGN(4)
  {
    *(.text .text.*)
    *(.rodata .rodata.*)
  } > IRAM
  
  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data .data.*)
    _edata = .;
  } > DRAM
  
  .bss (NOLOAD) : ALIGN(4)
  {
    _sbss = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;
  } > DRAM
  
  .rtc_fast.text : ALIGN(4)
  {
    *(.rtc_fast.text)
  } > RTC_FAST
  
  /DISCARD/ :
  {
    *(.eh_frame)
    *(.eh_frame_hdr)
  }
}
