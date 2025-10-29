MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 192K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(_estack);           /* 0: Initial stack pointer */
    LONG(Reset);             /* 1: Reset handler */
    LONG(0);                 /* 2: NMI */
    LONG(0);                 /* 3: HardFault */
    LONG(0);                 /* 4: MemManage */
    LONG(0);                 /* 5: BusFault */
    LONG(0);                 /* 6: UsageFault */
    LONG(0); LONG(0); LONG(0); LONG(0); /* 7-10: Reserved */
    LONG(0);                 /* 11: SVCall */
    LONG(0);                 /* 12: Debug Monitor */
    LONG(0);                 /* 13: Reserved */
    LONG(0);                 /* 14: PendSV */
    LONG(SysTick_Handler);   /* 15: SysTick - BURADA! */
  } > FLASH

  .text :
  {
    *(.text*)
    *(.rodata*)
  } > FLASH

  .data : AT(ADDR(.text) + SIZEOF(.text))
  {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM

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
