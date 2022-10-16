import os
import subprocess

import matplotlib.pyplot as plt
from sys import platform

from data_generator import *

POINTS_TYPE_CIRCLE = 'circle'
POINTS_TYPE_SQUARE = 'square'
POINTS_TYPE_CURVE = 'curve'


def main():
    points_range = range(1100, 10000, 1000)

    benchmark(points_range, POINTS_TYPE_CIRCLE)
    benchmark(points_range, POINTS_TYPE_SQUARE)
    benchmark(points_range, POINTS_TYPE_CURVE)


def benchmark(points_range, points_type):
    y_axis = []
    graham_scan_results = []
    gift_wrapping_results = []
    for i in points_range:
        result = test(i, points_type)
        if not result:
            print(f'Skipped {points_type}-{i}')
            continue
        gs, gw = result
        y_axis.append(i)
        graham_scan_results.append(gs)
        gift_wrapping_results.append(gw)

    plt.plot(y_axis, graham_scan_results)
    plt.plot(y_axis, gift_wrapping_results)
    plt.title(f'{points_type}, {points_range}')
    plt.show()


def generate_file(amount, points_type):
    points_file = data_file_name(amount, points_type)

    if os.path.isfile(points_file):
        # already generated
        return

    # predictable seed
    random.seed(amount)

    if points_type == POINTS_TYPE_CIRCLE:
        points = generate_points_circle(amount)
    elif points_type == POINTS_TYPE_SQUARE:
        points = generate_points_square(amount)
    elif points_type == POINTS_TYPE_CURVE:
        points = generate_points_curve(amount)
    else:
        raise Exception(f'Type {points_type} is not supported')

    f = open(points_file, "w")
    for p in points:
        f.write(f'{p[0]} {p[1]}\n')
    f.close()


def test(amount, points_type):
    generate_file(amount, points_type)
    result = run_rust('benchmark', data_file_name(amount, points_type))
    if result.startswith("OK"):
        split = result.split(" ")
        gs = split[1].removeprefix("gs=")
        gw = split[2].removeprefix("gw=")
        return int(gs) / 1000.0, int(gw) / 1000.0
    print(f'Test {points_type}-{amount} failed: {result.strip()}')
    return None


def run_rust(action, data_file):
    if platform == "win32":
        command = '../Handin2/target/debug/Handin2.exe'
    else:
        raise Exception(f'TODO: "{platform}" is not supported')

    output = subprocess.check_output([command, action, data_file])
    return output.decode("utf-8")


def data_file_name(amount, points_type):
    return f'data\\data_{points_type}_{amount}.txt'


if __name__ == '__main__':
    main()
