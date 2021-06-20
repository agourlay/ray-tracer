# ray-tracer

Implementation in Rust of a ray tracer following [The Ray Tracer Challenge](https://www.goodreads.com/book/show/39933047-the-ray-tracer-challenge) book.

```
The renderer is a ray tracer, which means it simulates the physics of light by tracing the path of light rays around your scene. 
Each exciting chapter presents a bite-sized piece of the puzzle, building on earlier chapters and setting the stage for later ones.
Requirements are given language-agnostically; it’s up to you to translate them into tests and code using whatever language you prefer.
When the project is complete, you’ll look back and realize you’ve built an entire system test-first!
```

## Status 

- [x] Chapter 1 - Tuples, Points, and Vectors
- [x] Chapter 2 - Drawing on a Canvas
- [x] Chapter 3 - Matrices
- [x] Chapter 4 - Matrix Transformations
- [x] Chapter 5 - Ray-Sphere Intersections
- [x] Chapter 6 - Light and Shading
- [x] Chapter 7 - Making a Scene
- [x] Chapter 8 - Shadows
- [x] Chapter 9 - Planes
- [ ] Chapter 10 - Patterns
- [ ] Chapter 11 - Reflection and Refraction
- [ ] Chapter 12 - Cubes
- [ ] Chapter 13 - Cylinders
- [ ] Chapter 14 - Groups
- [ ] Chapter 15 - Triangles
- [ ] Chapter 16 - Constructive Solid Geometry (CSG)
- [ ] Chapter 17 - Next Steps
- [ ] A1 - Rendering the Cover Image

## Examples

- chapter 8 on shadows.
![alt text](scenes/chapter-8.png "Chapter 9")

- chapter 9 on planes.
![alt text](scenes/chapter-9.png "Chapter 9")

## Hacking

### Performance profiling

The code can be profiled to remove bottlenecks and keep the rendering fast as features are added.

`cargo install flamegraph` to install https://github.com/flamegraph-rs/flamegraph

then inside the project directory we can profile `cargo run --release` by running `cargo flamegraph`

Latest [flamegraph](flamegraph.svg).

### Todos and ideas

[todo](todo.md)