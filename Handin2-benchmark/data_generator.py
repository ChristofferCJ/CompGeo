import random
import math


def generate_points_square(amount):
    points = []

    while len(points) < amount:
        x = random.randrange(0, 100)
        y = random.randrange(0, 100)

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
    points = []

    while len(points) < amount:
        x = random.randrange(0, 100)
        y = x*x

        points.append((x, y))
    return points
