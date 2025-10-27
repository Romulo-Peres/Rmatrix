<div align="center">
    <img width="100%" src="./assets/project image.png">
    <h1></h1>
    <h3>A terminal-based Matrix written in Rust with customizable features.</h3>
</div>
                                                                                  
<div align="center">
    <img width="100%" src="./assets/rmatrix.gif">
</div>                   
<div align="center">                                                  
The Rmatrix project leverages advanced features of modern terminals to simulate the iconic matrix effect. It provides customizable options, including color schemes, speed adjustments, and various visual behaviors.</div>

## Features
<ul>
    <li>Customizable stream trail color</li>
    <li>Customizable stream head color</li>
    <li>Configurable stream generation interval</li>
    <li>Independent speed for different streams</li>
    <li>Rainbow mode</li>
</ul>

## Table of contents
[Building the software](#building-the-software)

[Understanding the architecture behind the scenes](#understanding-the-architecture-behind-the-scenes)

[Base functionalities](#base-functionalities)

 - [Costumizing the stream trail color](#costumizing-the-stream-trail-color)

 - [Costumizing the stream head color](#costumizing-the-stream-head-color)

 - [Configuring the stream generation interval](#configuring-the-stream-generation-interval)

 - [Stream fall speed](#stream-fall-speed)

[Rainbow mode](#rainbow-mode)

[Exit the program](#exit-the-program)

## Building the software 🛠️
The Rmatrix is a Rust-based application, so its dependencies and compilation are fully managed by Cargo.

### Compiling
You can use the following command to compile the application. Run it from the project root directory:
```sh
cargo build --release
```
After that, the executable file ```rmatrix``` will be generated inside the ```target/release``` directory.


## Understanding the architecture behind the scenes ⚙️
### Head and trail
The Matrix effect implemented by this program is composed of a large number of streams. Each stream is divided into two parts: the **head** and the **trail**.

The **head** is the first character of a stream.

The **trail** consists of all characters that follow the head, with their color shaded based on how far they are from it.

The colors of the head and trail are customizable. Changing these values will cause all streams in the Matrix to use the new colors.

### Background streams
Background streams are a visual trick implemented in this project to create a 3D effect in the terminal. Without them, the Matrix would look too flat, breaking the immersion.

### Description:

New streams have a 50% chance of being background streams.

Background streams are the slowest streams (based on the value of the [maximum-stream-delay](#stream-fall-speed) configuration) and are always darker than the other streams.

## Base functionalities 🪛
After building the source code, the program is ready for use. Running it without any arguments will use the default configuration:

```sh
./rmatrix
```

Rmatrix also supports several arguments that let you adjust the program to your preferences.


### Costumizing the stream trail color
By default, the color of the stream trail is **green**. However, you can use the following argument to set it to any other color:

```sh
--trail-color or -b <RED> <GREEN> <BLUE>
```

The command above accepts an RGB value, so only numbers in the ```0–255``` range are valid.

### Costumizing the stream head color
By default, the color of the stream head is **white**, but the following argument allows you to change it:
```sh
--head-color or -e <RED> <GREEN> <BLUE>
```

Only numbers in the ```0–255``` range are valid.

### Configuring the stream generation interval
By default, a new stream is generated 20 milliseconds after the previous one. This delay works well for average-size screens, but it may be too long for smaller screens or too short for larger ones.

The following argument allows you to set a custom generation delay:
```sh
--stream-interval or -n <value in ms>
```

### Stream fall speed
In addition to background streams, each newly created stream has its own fall speed. Depending on the value set for this configuration, you can create a cool and dynamic Matrix effect.

The fall speed of a new stream is chosen randomly within a range, which defaults to ```40–90``` milliseconds.

You can update this range using the following arguments:
```sh
--minimum-stream-delay or -m <MINIMUM_STREAM_DELAY>

--maximum-stream-delay or -M <MAXIMUM_STREAM_DELAY>
```

**Notes:** Both arguments expect values in milliseconds. The **maximum-stream-delay** must always be greater than the **minimum-stream-delay**.

## Rainbow mode 🌈
Enabling rainbow mode will assign a random trail color to each stream when it is created, making the terminal very colorful. The color of the stream head is always white.

To enable rainbow mode, use the following argument when running the program:
```sh
--rainbow-mode or -a
```

## Exit the program 🚪
To exit the program while it is running, simply press Enter. The expected behavior is that the matrix will disappear, and all terminal settings will be reset to their default.
