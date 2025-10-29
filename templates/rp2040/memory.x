MEMORY
{
  /* RP2040 Memory Layout */
  /* Boot ROM: 0x00000000 - 0x00004000 (16K, not directly accessible) */
  /* SRAM: 264 KB total, divided into banks */
  /* Flash: External QSPI, accessed via XIP (Execute-in-Place) */
  
  BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

/* Stack pointer initial value (end of RAM) */
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  /* Second stage bootloader (required for RP2040) */
  .boot2 ORIGIN(BOOT2) :
  {
    KEEP(*(.boot2*));
  } > BOOT2

  /* Vector table */
  .vector_table ORIGIN(FLASH) :
  {
    LONG(_estack);           /* Initial stack pointer */
    LONG(Reset);             /* Reset handler */
    /* Additional interrupt vectors can be added here */
  } > FLASH

  /* Program code and constants */
  .text :
  {
    *(.text*)
    *(.rodata*)
  } > FLASH

  /* Initialized data */
  .data : AT(ADDR(.text) + SIZEOF(.text))
  {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM

  /* Uninitialized data */
  .bss :
  {
    _sbss = .;
    *(.bss*)
    *(.sbss*)
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.ARM.extab*)
  }
}
