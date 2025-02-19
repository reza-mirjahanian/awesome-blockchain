Solidity on Ethereum is a great language of choice for blockchain applications because, among other reasons, it is a completely deterministic programming language. However, it's also possible to create deterministic applications using existing popular languages like Java, C++, Python, or Go, by avoiding sources of non-determinism such as:

-   random number generators (without deterministic seeding)
-   race conditions on threads (or avoiding threads altogether)
-   the system clock
-   uninitialized memory (in unsafe programming languages like C or C++)
-   [floating point arithmetic](http://gafferongames.com/networking-for-game-programmers/floating-point-determinism/)
-   language features that are random (e.g. map iteration in Go)

While programmers can avoid non-determinism by being careful, it is also possible to create a special linter or static analyzer for each language to check for determinism.