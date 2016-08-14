A first class prometheus reporter written in rust. This is still an early work in progress. 
This library is currently bundled with metrics but we intend on breaking it out into its own
project as soon as it is stabilized. Rust metrics uses this library for prometheus integration.
The overarching goal is a library to provide prometheus support to any rust application or library.
We use the model having a seperate system thread for metrics collection.
