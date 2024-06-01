import torch.nn.functional as F
import torch.nn as nn
import torch

class MLP(nn.Module):
    def __init__(self):
        super().__init__()
        self.input = nn.Linear(784, 256, bias=True, dtype="f32")
        self.h1 = nn.Linear(256, 512, bias=True, dtype="f32")
        self.some_stuff = 1111
        self.relu1 = nn.ReLU(False)
        self.output = nn.Linear(512, 1, bias=True, dtype="f32")

    def forward(self, input_0) -> torch.Tensor:
        input_out = self.input(input_0)
        input_out_relu_func = F.relu(input_out, inplace=False)
        h1_out = self.h1(input_out_relu_func)
        relu1_out = self.relu1(h1_out)
        output_out = self.output(relu1_out)
        output_0 = F.sigmoid(output_out)
        print(f"Output: {output_0} | Num layers: {self.num_layers}")

        return output_0

class MLPModel(nn.Module):
    def __init__(self, int: int):
        super().__init__()
        self.num_layers = num_layers
        self.mlp = MLP({num_layers})

    def forward(self, input_0) -> torch.Tensor:
        output_0 = self.mlp(input_0)

        return output_0
