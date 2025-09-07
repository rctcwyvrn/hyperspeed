# lily's ultra low latency interface is...
## a microarchitecture
- aggressive caching
- multicore

## an ISA (lily's parallel architecture (LPA))
- runs things in parallel (think itanium)
- write a simulator (in rust) first

## a compiler (lily's immutable language (LIL))
- just a hyper-optimizing compiler
- extremely limited memory accesses 
- slam everything into a u64 -> language only exposes u8s as types
- as many non-growing basic data types as possible
- huge pre-allocations 
	- actually everything is just one giant heap/stack for the language since
- region based memory management (as static as possible)
- types/modes/effects that limit the ability to do things (for allowing for optimizations)
- only pure computations

a simple language
- only functions and structs

restrictions for speed
- there is no heap? or at least a very small heap that is only accessible when stack allocated data structures go over size
	- all numbers are unsized, but if they go over a i64 then they get slammed onto the stack
	- similarly for f64 floats
	- similarly for strings
		- three string types
			- small_string -> assumed to be less than 64b
			- string -> assumed to be less than 1kb
			- big_string -> assumed to be less than 1mb
	- no heap means no memory management, we get free regions by way of stack allocation
	- leave a canary on the stack to dealloc the 
- there are no mutable references, there is basic in-function mutation
	- all data must be passed by full ownership or by immutable ref
- you cant return references -> if you store the reference you cant return it anymore?????????

## a library OS (lily OS (LOS))
- OS monad that exposes all outside world interactions
	- works like a deferred, dropping back to the global OS executor
- global async executor 