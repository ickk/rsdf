WIP implementation of [Chlumsky's MSDF paper][msdf-paper].

[msdf-paper]: https://github.com/Chlumsky/msdfgen/files/3050967/thesis.pdf "Shape Decomposition for Multi-channel Distance Field"

Currently the core is capable of decomposing shapes made of lines, quadratic
bezier curvers & cubic bezier curves, however there is still work to do to
make it more capable and reliable.

![raster signed distance field output example](./shape.png)
![image rendered using the rsdf](./shape_render.png)

Future work:
 - implement additional primitives (arcs, b-splines, et c.)
 - implement front-end asset processors (svg, fonts)
