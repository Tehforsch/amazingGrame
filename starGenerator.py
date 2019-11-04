from math import cos, sin, pi
from itertools import zip_longest

OUTERRADIUS = 1
INNERRADIUS = 0
NUM = 3
step = 2*pi/NUM
outer = [[OUTERRADIUS*cos(step * x), OUTERRADIUS*sin(step * x)] for x in range(NUM)]
inner = [[INNERRADIUS*cos(step * x + step/2), INNERRADIUS*sin(step * x + step / 2)] for x in range(NUM)]
print(outer)
print(inner)
print("\n".join(str(y) +"," for y in [x for sublist in zip_longest(outer, inner) for x in sublist if x is not None]))
