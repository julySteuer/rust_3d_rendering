# 3D Rendering Framework
## What is this Framework
---
This Framework is a attempt of building a 3d framework on top of my already existing 2d engine. It can be used to display basic shapes or make your own, more complex, out of this given basic shapes.

## Technical Details 
---
### Libaries
---
````toml
winit = "0.25.0"
pixels = "0.6.0"
ndarray = "0.15.3"
````
winit: for opening a windows and capturing events like keystrokes </br>
pixels: for displaying single pixels in rgba </br>
ndarray: for linear algebra
### Build process
---
````shell
cargo run --release
````
Use this comand to run the main programm with the release optimisation, which is recommended because otherwise you are going to suffer from low fps
### Structure
---
````
|   Cargo.toml
|
\---src
    |   Circle.rs
    |   lib.rs
    |   Line.rs
    |   Mats.rs
    |   Polygon.rs
    |   Rectangle.rs
    |   Rotation.rs
    |   Vector.rs
    |   World.rs
    |
    \---dim3
            Camera.rs
            Cube.rs
            mod.rs
            Shape3d.rs
````
## Usage/Getting Started
---
### 1. Download it
Donwload the latest release on the right hand side of this github page if there is no version avilable just git clone this repo
````sh
git clone https://github.com/ToastBreadMan/rust_3d_rendering
````
### 2. Run it
```shell
cd rust_3d_rendering
cargo run --release 
```

### 3. Understand it
The documentation is at https://julysteuer.github.io/rust_3d_rendering/

## Development Update
A new version is being developed on the minifb branch. This version will be much faster (about 110 percent increase in the framerate) and easier to work with due to the use of minifb