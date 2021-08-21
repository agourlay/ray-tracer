# TODO

## Ideas
  
- cleanup float comparison in tests (typeclass for approximation with epsilon?)
- multi-threaded rendering
- add progress bar during rendering
- implement native operators + and * for my types
- promote a bunch of const marked as todos
- investigate Vec vs Array for modeling
- bench hot parts with https://bheisler.github.io/criterion.rs/book/index.html
- test target-cpu=native
  - RUSTFLAGS="-C target-cpu=native" cargo build --release
- advanced pattern suggestions as the end of chapter 10
  - radial gradient pattern
  - nested pattern
  - blended pattern
  - perturbed pattern
- use SIMD for math heavy part
  - matrix multiplication
  - vector dot product
  - https://www.youtube.com/watch?v=4Gs_CA_vm3o
- compare implementations
  - https://github.com/magnusstrale/raytracer
  - https://github.com/Dalamar42/rayt
  - https://github.com/sungiant/sdf
  
## More resources

- Writing a Ray tracer in Rust
  - https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/
  - https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
  - https://bheisler.github.io/post/writing-raytracer-in-rust-part-3/

- https://github.com/RayTracing/raytracing.github.io
  - https://raytracing.github.io/books/RayTracingInOneWeekend.html
  - https://raytracing.github.io/books/RayTracingTheNextWeek.html
  - https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html