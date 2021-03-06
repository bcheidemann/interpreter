# Usage

I created this project to learn more about how programming languages and interpreters
work. As such, it's not very useful! However, if you want to try it out for whatever
reason, simply clone the repo and run `cargo run` to run the REPL or `cargo run example.script`
to execute the example script.

# Example Code

```
print "######################################";
print "# Demo for interpreter version " + VERSION + " #";
print "######################################";
print "";

// Usage:
//   cargo run <shape> [...dimensions]
//
//   shape       cube | sphere
//   dimensions  (cube) <width> <height> <depth>
//               (sphere) <radius>
//
// Example:
//   cargo run cube 2 3 5
//   cargo run sphere 12

// Constants
Pi = 3.14159265359;

if ARG_2 == nil {
    print "Usage:";
    print "  " + ARG_0 + " " + ARG_1 + " <shape> [...dimensions]";
    print "";
    print "  shape       cube | sphere";
    print "  dimensions  (cube) <width> <height> <depth>";
    print "              (sphere) <radius>";
    print "";
}

if ARG_2 != nil {
    if ARG_2 == "cube" {
        width = ARG_3;
        height = ARG_4;
        depth = ARG_5;


        if width == nil print "Please provide a width argument";
        if height == nil print "Please provide a height argument";
        if depth == nil print "Please provide a depth argument";

        if width != nil if height != nil if depth != nil {
            volume = width * height * depth;
            print "The volume of a " + width + "m " +
                                "x " + height + "m " +
                                "x " + depth + "m " +
                                "cube " +
                                "is " + volume + "m^3";
        }
    }

    if ARG_2 == "sphere" {
        radius = ARG_3;

        if radius == nil print "Please provide a radius argument";

        if radius != nil {
            volume = (4/3)*Pi*radius*radius;
            print "The volume of a sphere with radius " + radius + "m " +
                                                  "is " + volume + "m^3";
        }
    }

    if ARG_2 != "cube"
        if ARG_2 != "sphere"
            print "Unknown shape: " + ARG_2;
}
```
