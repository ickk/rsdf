WIP implementation of [Chlumsky's MSDF paper][msdf-paper].

[msdf-paper]: https://github.com/Chlumsky/msdfgen/files/3050967/thesis.pdf "Shape Decomposition for Multi-channel Distance Field"

Currently the core is capable of decomposing shapes made of lines, quadratic &
cubic bezier curves, however there is still work to do to make it into a real
tool.

Example multichannel raster-SDF generated using this library:

![raster-SDF example](./shape.png)

A 900x900 pixel image rendered using the above 30x30 pixel SDF image:

![image rendered from raster-SDF](./shape_render.png)

Future work:
- [ ] implement additional primitives (arcs, b-splines, et c.)
- [ ] implement front-end asset processors (svg, fonts)
- [ ] try to parallelise the core with rustgpu
