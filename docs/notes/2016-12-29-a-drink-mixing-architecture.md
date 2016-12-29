# An Architecture For Pumping Drinks

## Starting off the day with some learning about Rust!

I started off the day off by watching the [into_rust()](http://intorust.com)
video lectures and exercises! These really helped me improve my understanding of
Rust's Ownership model. My understanding was still stuck in C/C++!

## Architecture

Let's start-off by making a little bit of an architecture! The following
picture was taken from a quick sketch I did, but I believe it has the
core pieces. This architecture only goes up to the `web api`. The front-end
website will be dealt with later.

![Architecture](../assets/images/architecture/rusty-nail-architecture-001.svg)

Some terms that will be used in the project:

- `dispenser`: Is an abstraction of the pump. it maintains state of how much liquid is in it, how to pump it at a given rate, and how to pump a given quantity. This has no idea what it is dispensing, but it can dispense it.
- `recipe`: A recipe is a listing of all of the liquids which need to be mixed (by name) and how much of each of them to make a givent quantity (in _mL_). Recipes also include information about user interactions, such as, if the drink should be mixed or stirred, or if ice should be added. Recipes have names and cam be looked up.
- `mixer`: A `mixer` contains a list of named `dispensers`. The `dispensers` get there name from a configuration file, or from the `api`, the `mixer` also sends down information about the liquid levels. A `mixer` creates a drink by taking a `recipe` and `quantity` as input. System status information is relayed through the `mixer` to the LEDs

The goal is to create a service which will provide an HTTP API using one of
Rust's web micro-frameworks (
[Iron](https://github.com/iron/iron),
[pencil](https://github.com/fengsp/pencil),
[rocket](https://github.com/SergioBenitez/Rocket), etc.). After that is tested
I will begin work on a basic frontend.
