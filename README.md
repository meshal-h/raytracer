# Ray Tracer

A simple ray tracer following the amazing tutorial by Peter Shirley et al. ["Ray Tracing in One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html). Implementation is done in Rust.

### Examples
<img src="media/penultimate_scene.png" width=80%>

<img src="media/final_scene.png" width=80%>

### Runtime Performance
Rendering the final scene (1200x675 image with 500 samples per pixel and 50 max depth) on an AMD Ryzen 9 5900HX CPU and 16GB of RAM:

| Implementation         | Time        |
|------------------------|-------------|
| Rust (multi-threaded)  | 2min 21sec  |
| Rust (single-threaded) | 16min 53sec |
| C++ (single-threaded)  | 27min 31sec |

- Multi-threading in Rust is done through Rayon, which shows an excellent scaling on an 8-core CPU. The current code is multi-threaded, and the single-threaded code can be obtained by changing ``into_par_iter`` to ``into_iter`` and ``ParallelProgressIterator`` to ``ProgressIterator`` inside *camera\.rs*.
- The C++ is the [reference implementation](https://github.com/RayTracing/raytracing.github.io) complied with GCC 11.4 (which was ~10% faster than the code complied with Clang 14). This implementation prioritizes simplicity and having fewer dependencies over performance.
- A notable difference between the implementations is that the C++ one uses float64 while I use float32 in Rust. Further, the C++ implementation allows the same material to be referenced by more than one object in the scene.
