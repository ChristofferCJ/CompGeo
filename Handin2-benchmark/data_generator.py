import random
import math


def generate_points_square(amount):
    points = []

    while len(points) < amount:
        x = random.random() * 100
        y = random.random() * 100

        points.append((x, y))
    return points


def generate_points_circle(amount):
    points = []

    while len(points) < amount:
        r = math.sqrt(random.random()) * 100
        theta = random.random() * 2 * math.pi
        x = r * math.cos(theta)
        y = r * math.sin(theta)

        points.append((x, y))
    return points


def generate_points_curve(amount):

    used_x = set()
    points = []

    while len(points) < amount:
        x = random.randrange(0, 1000000) / 1000.0
        y = x*x

        if x in used_x:
            continue
        else:
            used_x.add(x)
            points.append((x, y))
    return points
