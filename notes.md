# Notes on ray tracing

## Code structure
- `Vec3` class: 

## PPM files
Reference: https://en.wikipedia.org/wiki/Netpbm

The P6 image format works the following way:

```
P3 

C R
MAX_COLOR
R G B R G B R G B
....
```

Where `C` is amount of columns, `R` is amount of rows. `MAX_COLOR` is the set to 255 usually.

## Rays
To render a scene using a tracer, all of the rays, which spawn from a specific light source, must be traced at every position in the 3D space. A computation of what color is seen at every position must also take place, to color the scene.

The ray can be thought of as a function $P(t) = A + tb$ where $P$ is the position of the ray, $A$ is the origin and $b$ is ray's direction. $P$ moves along the ray for all real values of $t$. The parts in 'front' of the ray are obtained for all positive $t$ and constructs, what we call a _ray_.
