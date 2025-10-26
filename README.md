<div align="center">
    <img width="100%" src="./assets/project image.png">
    <h1></h1>
    <h3>A terminal-based Matrix simulator written in Rust with customizable features.</h3>
</div>
                                                                                  
<div align="center">
    <img width="100%" src="./assets/rmatrix.gif">
</div>                   
<div align="center">                                                  
The (Remastered) R-matrix uses modern terminal capabilities to simulate a Matrix effect. Its customizable features let you fine-tune the visuals and behaviors for a more personalized experience.
</div>

## Features
<ul>
    <li>Customizable stream trail color</li>
    <li>Customizable stream head color</li>
    <li>Configurable stream generation interval</li>
    <li>Independent speed for different streams</li>
    <li>Rainbow mode</li>
</ul>

## Table of contents
[Compiling the project](#compiling-the-project)

[Running the R-matrix](#running-the-r-matrix)

[Custom colors](#custom-colors)

 - [Matrix body color](#matrix-body-color)

 - [Matrix head color](#matrix-head-color)

[Custom behaviors](#custom-behaviors)

 - [String interval](#string-interval)

 - [Render speed](#render-speed)

[Exit the program](#exit-the-program)

## Compiling the project
The compilation of the project can be simply done using the cargo build command. Assuming you are at the root of the project, the following command should do the trick:
```sh
cargo build --release
```

## Running the R-matrix

By default the program runs by just calling its executable that is generated inside the `target/release/` directory:
```sh
./rmatrix
```

## Custom colors
R-matrix uses green by default, but you can customize it with your preferred colors.

### Matrix body color
Setting a new value for the matrix body color will change the color of the strings rendered on screen.
```sh
rmatrix --body-color <R> <G> <B>
```

<b>Default value: 0 255 0 (Green)</b>

    
### Matrix head color
Setting a new value for the matrix head color will change the color of the first character in all matrix strings.
```sh
rmatrix --head-color <R> <G> <B>
```
<b>Default value: 255 255 255 (White)</b>


## Custom behaviors
R-matrix also allows you to set the speed at which new strings are generated and how quickly they are rendered on screen.

### String interval
Setting a new value for the string interval will adjust the delay for creating a new R-matrix string.

```sh
rmatrix --string-interval <Ms>
```
<b>Default value: 20 (milliseconds)</b>

### Render speed
Setting a new value for the render speed will change how quickly the R-matrix strings are pulled down.
```sh
rmatrix --render-speed <Ms>
```
<b>Default value: 30 (milliseconds)</b>

## Operation modes
R-matrix offers additional modes beyond the default one. Below is a detailed section explaining how they work.

### Rainbow mode
The rainbow mode generates strings with random body colors, making your terminal and, possibly your setup, vibrant and colorful. The streams' head are set to bright white `(255, 255, 255)`.

The rainbow mode can not have its colors changed by any flag.

To enable rainbow mode at program startup, use the `--rainbow-mode` flag.
```sh
rmatrix --rainbow-mode
```

## Exit the program
To exit the program while it is running, simply press Enter. The expected behavior is that the matrix will disappear, and all terminal settings will be reset to their default configuration.
