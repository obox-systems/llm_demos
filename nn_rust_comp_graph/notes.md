# [Neural Networks in Rust: Computation Graph](https://www.youtube.com/watch?v=DGVoGK-gtjQ&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=18)

## Purpose

In Machine Learning it is neccesary to calculate gradients of functions in order to perform a gradient decent.

A clever way to do this is to trace all math operations that were in the code, construct a computational graph, and then based on that graph - derive the derivative.

## How to implement this

It is simple:

You need to have a `Value` representation that can be either a literal value, an unary operation of some other `Value` or a binary operation.

Tracing operation is easy: you just need to implement traits from `std::ops` like `Add` and others.
