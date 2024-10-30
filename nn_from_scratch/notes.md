# [Neural Networks From Scratch in Rust](https://www.youtube.com/watch?v=DKbz9pNXVdE&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=15)

## Basic workings of neural networks

- NN consists of several stages
- You supply output to NN. This is called forward pass.
- The output is not ideal. You apply an error/cost function to estimate how good the resuls it.
- You analyze which sections affected how much the output and tweak it. This is called backpropagation.

In short all of this is:

Forward pass - cost function - backpropagation.

## How neuron works

1. The neuron sums all outputs.
2. Each output has a weight: a coefficient.
3. That sum is added with a bias (just a single parameter).
4. The result is passed on to the activation function.
5. The result of the activation function is the output of a neuron.

The activation function is the reason NNs have such good capabilities of approximation. They allow the output of NN to be non-linear.

## How neurons are organized

They are organized in layers. There are 3 types of layers:

- Input layer: they are fake neurons as they don't have weights, biases, and activation functions.
- Hidden layer: they are true neurons.
- Output layer: also true neurons. Output of those neurons is the output of NN.

Each neuron in layer N is connected to all neurons of layer N-1.

## Mathematical representation of a NN

It's tedious to analyze NN by single neuron. *IMHO: actually, the more you study this, the less you call it a neural network. It's just a mathematical model with a lot of tweakable paramters.*

And so, it's more focused on layer:

- Inputs to layer can be represented as a vector.
- Weights is a matrix, collumn count of which is equal to length of the output vector, and row count is the length of the output vector.
- Biases can be also represented as a vector of length of count of neurons in a layer.
- Activation function is a function.

Now, the output of a layer is represented like this:

$ \hat{y} = A(I * W + B) $

Where $\hay{y}$ is the output of a network, $A$ is the activation function, $I$ - input vector, $W$ - weights, $B$ - biases.

It is also possible to pass batches of input at once by representing the input not as a vector, but as a matrix.
