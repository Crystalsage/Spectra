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

![](./figures/lerp.png)

As said, the tracer sends rays through pixels and computes colors that are seen at every pixel. The steps are as follows:

1. Calculate ray from the eye to pixel.
2. Determine the objects which the ray intersects.
3. Compute a color for the intersection point.


### Scene: Blending colors 
To create a gradient background, we can make a function that blends colors together to give a us smooth gradient. We can achieve this using linear interpolation.

$blendedValue = (1 - t) \cdot startValue + t \cdot endValue$

When $t = 0.0$, we'd like blue color. When $t = 1.0$, we'd like white color. Everywhere in between, we'd like a blend that varies, according to the $y$ co-ordinate in this case, because we would like a vertical gradient.

The result is:
![](./figures/ray_gradient_lerp.png)
