/* Linker script for the STM32F303 found on the DISCOVERY board */
MEMORY
{
	FLASH : ORIGIN = 0x08000000, LENGTH = 256K
	RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
