from typing import List
Block = List[float]

def quicksort(blocks: List[Block], left_idx: int, right_idx: int):
    if (right_idx - left_idx) < 0:
        return blocks
    last_block = read_non_adjacent_block(blocks, right_idx)
    pivot = last_block[-1]
    partition_idx = partition(blocks[left_idx:right_idx], pivot)
    quicksort(blocks, left_idx, partition_idx-1)
    quicksort(blocks, partition_idx+1, right_idx)