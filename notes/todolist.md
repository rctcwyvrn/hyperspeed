# hello world
- [ ] read https://queue.acm.org/detail.cfm?id=3639445 and  https://queue.acm.org/detail.cfm?id=2544374 and https://www.ralfj.de/blog/2018/07/24/pointers-and-bytes.html and https://www.ralfj.de/blog/2020/12/14/provenance.html and https://faultlore.com/blah/oops-that-was-important/ and https://faultlore.com/blah/tower-of-weakenings/
- [ ] think about the implications of itanium and what a language targetting itanium would look like
- [ ] design a basic ISA for LPA
- [ ] write a basic simulator for LPA in rust
- [ ] write the basic low level nanopasses for LIL
- [ ] write a basic library for writing bytes out to serial (probably in raw LPA)
- [ ] write a hello world program in low-level-LIL
- [ ] think really hard about the LIL type system
- [ ] think really hard about the medium levels of the compiler and what it would look like (how to make something go very fast that can still write an entire OS in)
- [ ] think really hard about if we need memory management if there's no heap or storing references 
- [ ] write the optimization layers of the compiler, read https://www.clear.rice.edu/comp512/Lectures/Papers/1971-allen-catalog.pdf
- [ ] write the lowering layers of the compiler
- [ ] write the parser and lexer 
- [ ] write a hello world program in real LIL 

# hello hardware
- [ ] read https://ceramichacker.com/blog/category/hardcaml-mips
- [ ] implement LPA in hardcaml
- [ ] design a basic ABI (serial in/out)
- [ ] flash LPA to an arty a7 and run the hello world program

# hello internet
- [ ] implement ethernet parts of the ABI (layer 1)
- [ ] implement the basic packet level parts of LOS (layer 3)
- [ ] implement ip and dns and bgp and whatever else is required (layer 3)
- [ ] implement tcp (layer 4)
- [ ] implement TLS (layer 5)
- [ ] implement http parsing 
- [ ] write a hello world https site

# hello speed
- [ ] write more optimization passes (?)
- [ ] benchmark (how to benchmark a webserver? https://crates.io/crates/oha)