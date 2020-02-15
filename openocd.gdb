# TODO: figure out what this means exactly and use Mara's
target extended-remote :3333
#target extended-remote | openocd -f openocd.cfg

# print demangled symbols
set print asm-demangle on

# enable printing output from the target on host
monitor arm semihosting enable

# TODO ask what this means
# monitor arm semihosting_fileio enable

load

monitor reset halt

stepi
