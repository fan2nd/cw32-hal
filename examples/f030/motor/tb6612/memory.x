/* CW32F030C8T7 only. Verified chip memory metadata:
 * ../../../../cw32-data/data/chips/CW32F030C8T7.json
 * FLASH starts at 0x00000000, SRAM starts at 0x20000000.
 * Do not reuse this map for an F6 device or a bootloader-offset application.
 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}
