# Booth's multiplication algorithm #############################################

[Booth's multiplication
algorithm](https://en.wikipedia.org/wiki/Booth%27s_multiplication_algorithm)
implementation written in Rust.

Take two 16-bit signed integers (values allowed are from -32767 to 32767, as
-32768 is forbidden here) and multiply them by using bitwise trickery.

:warning: The outputed value is also a 16-bit signed integer, it will overflow
if the result goes out of the [-32768, 32767] range.
