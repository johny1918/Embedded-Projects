/* Memory layout for STM32F407VGTx (for example STM32F407VGT6). */
MEMORY
{
  /* Program code and read-only data. */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K

  /* SRAM1 (112 KiB) and SRAM2 (16 KiB) form one contiguous region. */
  RAM : ORIGIN = 0x20000000, LENGTH = 128K

  /* Core-coupled RAM; not used by the default cortex-m-rt sections. */
  CCMRAM : ORIGIN = 0x10000000, LENGTH = 64K
}
