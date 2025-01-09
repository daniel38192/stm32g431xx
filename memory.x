/* linker script for STM32H562RGT6 */
MEMORY
{
  RAM    (xrw)    : ORIGIN = 0x20000000,   LENGTH = 640K
  FLASH    (rx)    : ORIGIN = 0x08000000,   LENGTH = 1024K
}