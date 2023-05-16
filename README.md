<p align="center" width="100%">
  <img src="./rsdf_render.png?raw=true" alt="rsdf logo">
</p>

A **raster signed distance field** generator

---

This project is an implementation of the technique outlined in
[Chlumsky's MSDF thesis][chlumsky-paper], which is an improvement on
[Valve's technique][valve-paper].

[chlumsky-paper]: https://github.com/Chlumsky/msdfgen/files/3050967/thesis.pdf "Shape Decomposition for Multi-channel Distance Field"
[valve-paper]: https://cdn.akamai.steamstatic.com/apps/valve/2007/SIGGRAPH2007_AlphaTestedMagnification.pdf "Improved Alpha-Tested Magnification for Vector Textures and Special Effects"

---

### Status

Currently the `rsdf_core` crate is capable of decomposing shapes made of
lines, quadratic/cubic bezier curves, and elliptical arcs,  however there is
still work to do to make it into a useful tool.

The logo above was rendered from the following multichannel SDF image, which
was generated using this codebase:

![raster-SDF example](./rsdf.png?raw=true)

---

### Future work

- [ ] implement additional primitives
  - [x] elliptic & circular arcs
  - [ ] b-splines
- [ ] implement front-end asset processors (svg, fonts)
- [ ] try to parallelise the core with rustgpu
