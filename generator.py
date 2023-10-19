from uuid import uuid4
from random import uniform, gauss
from math import floor

dicts = []

print("id,value")

for x in range(10_000_000):
    x = uniform(0, 1)
    offset = 32
    index = int(floor(offset / x)) - offset

    while len(dicts) <= index:
        dicts += [(gauss(0, 1), abs(gauss(0, 1)), uuid4())]

    print("{},{}".format(dicts[index][2], gauss(dicts[index][0], dicts[index][1])))
