https://x.com/fu5ha/status/1711336181508518006

![[Screenshot_20250906-165926.png]]

What do I envision?
- extremely limited abstractions 
- extremely limited data types
- extremely limited memory accesses 

Slam everything into a u64 -> language only exposes u8s as types

As many non-growinf basic data types as possible

Huge pre-allocations 

Region based memory management (as static as possible)

Types/modes/effects that limit the ability to do things (for allowing for optimizations)

Pure, eager language, IO monad, no side effects allowed ever 

No FFI, only pure computations

OS monad that exposes all outside world interactions

Everything is multicore/multiprocess by default?
- Does this involve building my own CPU and isa?
- a hardcaml cpu with my own ISA would be sick
- woodpecker inspired ISA without conditionals? A compiler that compiles to minimal conditions to improve prefetching?

# Lily's arithmetic machine
- everything is multicore
- everything is avx
- LAM (Lily's arithmetic machine)

# lily's ultra low latency interface
- LULLI
- usb plugged into machine, ethernet plugged into router
- aspire gets rolled into this, you compile your program into the OS and flash it onto the FPGA

designing an isa
https://queue.acm.org/detail.cfm?id=3639445

designing an IR
https://queue.acm.org/detail.cfm?id=2544374