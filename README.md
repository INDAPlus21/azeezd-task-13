# Azeez Daoud - Ray Tracer
Based on tutorial by Peter Shirley [Ray Tracer in One Weekend](https://raytracing.github.io/)

# Content
- [Running](#running)
    - [Optional Arguments](#optional-arguments)
- [Making a `.scene` file](#making-a-scene-file)
    - [Commands](#commands)
    - [Materials](#materials)
    - [Random Values](#random-values)
    - [Preview Render](#preview-render)
- [Progress](#progress)
- [Example Renders](#example-renders)

# Running
Rendering can be done by running 

`cargo run --release <scene file name> <optional arguments...>`

`<scene file name>` is the file path of the `.scene` file you want to render using the default ray tracing method. `<optional arguments...>` are given arguments to change the rendering settings.

## Optional Arguments

| Argument      | Description |
| ----------- | ----------- |
| `-@name`   | Provide the name of the output file by replacing the `name` with the desired name.        |
| `-s000`      |  Provide the amount of samples/pixels for the render by replacing the `000` with the desired amount. If no samples are given then the default of 100 is used         |
| `-b000` | Provide the amount of light bounces for the render by replacing the `000` with the desired amount. If no light bounces are given then the default of 10 is used |
| `-f`      | Render the objects of the scene without any ray tracing (used for fast preview of the scene). Suffixes the output file name with `_preview`       |

Here is an example of a render of a scene called `myScene.scene` that uses 1000 samples/pixel and 50 light bounces. The output file name is `myRender.png`

`cargo run --release scenes/myScene.scene -@myRender -s1000 -b50`

Here is an example of a preview of that same scene using the fast render

`cargo run --release scenes/myScene.scene -@myRender -f`

**NOTE!** The output file extension should not be provided because all output is in .png format

# Making a `.scene` file
A `.scene` file is where you can write how the scene should look like by defining materials, objects and camera information.

Every command should be on a new line. Whitespace is ignored (put as many tabs, spaces or new lines as you want).

See examples under [scenes](/scenes/) directory. You can see the rendered version under [Example Renders](#example-renders).

## Commands
There are 4 commands in total, `CAM`, `MAT`, `OBJ` (split into spheres and rectangle) and `~`
| Command | Description|
|----     |----        |
|`CAM f1 f2 f3 t1 t2 t3 fov` | Define the position of the camera. `f1 f2 f3` is the origin of the camera (`f` for from). `t1 t2 t3` is for the target of the camera (`t` for target or to). `fov` is the angle of the vertical field of view.|
| `MAT name type c1 c2 c3 v`  | Define a material by giving it a `name` and the `type`. `c1 c2 c3` is the colour of the material. `v` is other values for the given material. (Read [Materials](#Materials)) |
| `OBJ sphere materialName c1 c2 c3 r`       | Place a sphere whose center is at `c1 c2 c3` with a radius `r`. `materialName` is the name of materials you defined.   |
| `OBJ rect materialName axis a1 a2 b1 b2 d` | Place an axis aligned rectangle where `axis` is in what axis (`xy`, `xz` or `yz`) the rectangle is aligned with. `a1 a2` is the coordinates of the boundries in the first axis. `b1 b2` is the coordinates of the boundries in the second axis. `d` is the coordinate in the third (orthogonal to the given axis) axis. Example `OBJ rect matName xy -1 1 0.5 1.5 2` would place a rectangle with corners at (-1, 0.5, 2), (-1, 1.5, 2), (1, 0.5, 2) and (1, 1.5, 2). |
| `~ x`     | Repeats the next command x times |
| `//`       | Comment the code, should be on a seperate line and add a space after the `//`. |
### Random Values
You can provide random values for the coordinates or colours using the range operator `_`. To gain a random values in the range [x, y], use `x_y` instead of giving a single value

### Preview Render
The preview render is a render that colours each object based on a single ray cast that calculates the colour based on the normal of the object it hits.
This is as stated under [Optional Arguments](#optional-arguments) is when the `-f` argument is passed to the renderer.

Here is an example of the *materials.scene* Scene Render

|Preview using `-f` | Full Ray Casting |
|---                |---               |
|![materialsPreview](/images/materials_preview.png)| ![materials](/images/materials.png) |

## Materials
| Material  | Image  |
|-- |-- |
| `lambertian`  | ![lambertian](/images/materials/lambertian.png)  |
| `metal` (`v` is to provide fuzziness) | ![metal](/images/materials/metal.png)   |
| `dielectric` (`v` is to provide index of refraction) | ![dielectric](/images/materials/dielectric.png) |
| `light` `(c1 c2 c3)` can be over 1.0 to add more brightness| ![light](/images/materials/light.png) |       

# Progress
This is my progress through the ray tracer programming. What a journey...
|Step       | Image      |
|---    |---   |
| A wild sphere has appeared! |  ![prog0](/images/progress/trace_0.png)     |
| Sky is now sober |   ![prog1](/images/progress/trace_1.png)   |
| Small mistake makes messy light      |   ![prog2](/images/progress/trace_2.png)   |
| Fix the mistake and add materials | ![prog3](/images/progress/trace_3.png)   |
| Let there be light |     ![prog4](/images/progress/trace_4.png)     |

# Example Renders
Here are some example scenes they are rendered on my home device.
|Title       | Image       | Pixel Samples & Bounces | Render Time (on my device)   |
|--     |--     |--     |--     |
| Materials      |  ![materials](/images/materials.png)     | 1000 s/p, 50 bounces       |  9m 33s   |
| Mirrors | ![mirrors](/images/mirrors.png) | 1000 s/p, 50 bounces | 19m 8s |
| Spheres | ![spheres](/images/spheres.png) | 3000 s/p, 10 bounces | 14m 5s |
| Mix of All | ![mixOfAll](/images/mixOfAll.png) | 5000 s/p, 10 bounces | 8m 1s |
| Strange Refractions | ![refract](/images/strangeRefract.png) | 5000 s/p, 10 bounces | 9m 16s |
| Mirrors Mirrors| ![moreMirrors](/images/mirrorsMirrors.png) | 1000 s/p, 20 bounces | 9m 22s |
