# Booth's multiplication algorithm #############################################

Very simple [Booth's multiplication
algorithm](https://en.wikipedia.org/wiki/Booth%27s_multiplication_algorithm)
implementation written in Rust.

Take two 16 bit signed integers (allowing values from -32768 to 32767) and
multiply them by using bitwise trickery.

As the outputed value is also a 16 bits signed integer, it will rapidly overflow
when the result goes out of its limits.
