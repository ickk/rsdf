Currently non-functional WIP implementation of [Chlumsky's MSDF paper][msdf-paper].

[msdf-paper]: https://github.com/Chlumsky/msdfgen/files/3050967/thesis.pdf "Shape Decomposition for Multi-channel Distance Field"

Contours use a CCW-positive winding order. That is, contours within a shape
that are counter-clockwise are *additive* to the fill region of the shape,
where-as clockwise contours are *subtractive*.
