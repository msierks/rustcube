# Script to ease the debugging of Rustcube with GDB
#
# Run with:
#    gdb-multiarch -x rustcube.gdb

define rx
    set endian big
    target remote 127.1:9001
    #layout regs
end
document rx
    Connect to a local Rustube instance
end

define sd
    stepi
    disassemble/r $pc-40,+80
end
document sd
    Steps a single instruction and disassemble around PC
end
