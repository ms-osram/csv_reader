import csv
from math import pow

filename = 'input.csv'

entries = {}

# read the CSV file
with open(filename, newline='') as csvfile:
    reader = csv.reader(csvfile)
    next(reader) # skip header

    for row in reader:
        if row[0] not in entries:
            entries[row[0]] = []
        entries[row[0]].append(row[1])

summary = []
for (uuid, values) in entries.items():
    n = 0
    cumsum = 0
    ssq = 0

    # calculate mean
    for value in values:
        n += 1
        cumsum += float(value)
    mean = cumsum / n
    
    # calculate stdev
    for value in values:
        ssq = pow((mean - float(value)), 2)
    stdev = pow((ssq / n), 1./2)

    summary.append([uuid, n, mean, stdev])

# sort by size
summary.sort(key = lambda summary: summary[n], reverse = True)

for [uuid, n, mean, stdev] in summary:
    print("{}\t{}\t{}\t{}".format(uuid, n, mean, stdev))
