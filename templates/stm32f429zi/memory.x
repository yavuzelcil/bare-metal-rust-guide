MEMORY
{
  /* STM32F429ZI Memory Layout */
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM: ORIGIN = 0x10000000, LENGTH = 64K
}

/* Stack pointer initial value (end of RAM) */
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  /* Vector table at the start of FLASH */
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

  /* Initialized data (copied from FLASH to RAM at startup) */
  .data : AT(ADDR(.text) + SIZEOF(.text))
  {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM

  /* Uninitialized data (zero-initialized at startup) */
  .bss :
  {
    _sbss = .;
    *(.bss*)
    *(.sbss*)
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /* Discard debug info and other unnecessary sections */
  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.ARM.extab*)
  }
}
