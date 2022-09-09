import math
from typing import List

Block = List[float]

# Input: N numbers, laid out sequentially on the magnetic tape in blocks
# Assume M >= N + sizeof(number)
def find_minimum(blocks: List[Block]):
    minimum = -math.inf
    for block_idx in len(blocks):
        numbers = read_adjacent_block(blocks, block_idx)
        for number in numbers:
            if number < minimum:
                minimum = number
    return minimum