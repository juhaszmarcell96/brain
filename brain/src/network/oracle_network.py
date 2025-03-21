import numpy as np

seed = 7

def activation (x):
    return np.maximum(0, x)

input_neurons, hidden_neurons, output_neurons = 30, 100, 10

np.random.seed(seed)
w1 = np.random.rand(input_neurons, hidden_neurons) * 2.0 - 1.0
w2 = np.random.rand(hidden_neurons, output_neurons) * 2.0 - 1.0
b1 = np.random.rand(1, hidden_neurons) * 2.0 - 1.0
b2 = np.random.rand(1, output_neurons) * 2.0 - 1.0

input = np.random.rand(1, input_neurons)
hidden = activation(input.dot(w1) + b1)
output = activation(hidden.dot(w2) + b2)

print(f"// seed : {seed}")
print(f"// shape : {input_neurons}, {hidden_neurons}, {output_neurons}")

print(f"network.resize(0, {input_neurons});")
print(f"network.resize(1, {hidden_neurons});")
print(f"network.resize(2, {output_neurons});")

for i in range(input_neurons):
    for e in range(hidden_neurons):
        print(f"network.set_weight(0, {i}, {e}, {w1[i][e]});")

for i in range(hidden_neurons):
    for e in range(output_neurons):
        print(f"network.set_weight(1, {i}, {e}, {w2[i][e]});")

for i in range(hidden_neurons):
    print(f"network.set_bias(0, {i}, {b1[0][i]});")

for i in range(output_neurons):
    print(f"network.set_bias(1, {i}, {b2[0][i]});")

print(f"let mut input = Matrix::new(1, {input_neurons});")
for i in range(input_neurons):
    print(f"input[0][{i}] = {input[0][i]};")

print("let output = network.forward(&input);")
for i in range(output_neurons):
    print(f"assert!((output[0][{i}] - {output[0][i]}).abs() < 1e-9);")