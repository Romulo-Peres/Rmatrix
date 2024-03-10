<div align="center">
    <img width="100%" src="./assets/project image.png">
    <h1></h1>
    <h3>A matrix simulator for terminal interfaces written in Rust with customizable features</h3>
</div>
                                                                                  
<div align="center">
    <img width="100%" src="./assets/rmatrix.gif">
</div>                   
<div align="center">                                                  
    The (Remastered) R-matrix uses some features of the modern terminals to create the illusion of a matrix effect. The project has customizable features that make the matrix more attractive and behaves the way you like most.
</div>

# Features
<ul>
    <li>Set matrix body color</li>
    <li>Set matrix edge color</li>
    <li>Set string interval</li>
    <li>Set render speed</li>
</ul>

# Table of contents
[Compiling the project](#compiling-the-project)

[Running the R-matrix](#running-the-r-matrix)

[Custom colors](#custom-colors)

 - [Matrix body color](#matrix-body-color)

 - [Matrix edge color](#matrix-edge-color)

[Custom behaviors](#custom-behaviors)

 - [String interval](#string-interval)

 - [Render speed](#render-speed)

[Exit the program](#exit-the-program)

# Compiling the project
The compilation of the project can be simply done using the cargo build command. Assuming you are at the root of the project, call the following command:
```sh
cargo build --release
```

# Running the R-matrix

By default the program runs by just calling its executable that is generated inside the `target/release/` directory:
```sh
./rmatrix
```

# Custom colors
R-matrix allows you to set new colors for the matrix body and edges.

## Matrix body color
Set a new value to matrix body color will change the color of the strings rendered on screen

```sh
rmatrix --body-color <R> <G> <B>
```

The default RGB value to this feature is 0 255 0 (Green)

    
## Matrix edge color
Set a new value to matrix edge color will change the first character's color of all matrix strings

```sh
rmatrix --edge-color <R> <G> <B>
```
Default value to: 255 255 255 (White)


# Custom behaviors
R-matrix make it also possible to set the speed which new strings are generated and how quickly they are rendered on screen.

## String interval
Set a new value to string interval will change the delay to create a new Rmatrix string

```sh
rmatrix --string-interval <Ms>
```
The default value is 20 (milliseconds)

## Render speed
Set a new value to render speed will change the how quickly the Rmatrix strings are pulled down

```sh
rmatrix --render-speed <Ms>
```
Default to: 30 (millisecond)

# Exit the program
To exit the program while it is running, simply press Enter. The expected behavior is for the matrix to disappear and for all terminal configurations to be reset to default.

