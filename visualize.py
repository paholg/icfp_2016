#!/usr/bin/python2.7

import matplotlib.pyplot as plt
import random
import sys

if (len(sys.argv) != 2):
    print "Usage: ./visualize.py filename"
    exit(1)

shape_file_name = sys.argv[1]
shape_file = open(shape_file_name, "r")

### parse file ###
num_polys = int(shape_file.readline())

for _ in range(num_polys):
    X = []
    Y = []
    num_vertices = int(shape_file.readline())
    for _ in range(num_vertices):
        v_str = shape_file.readline()
        x = float(eval("1.0*" + v_str[:v_str.find(',')]))
        y = float(eval("1.0*" + v_str[v_str.find(',')+1:]))
        plt.plot(x, y, '.k', markersize=50)
        X.append(x)
        Y.append(y)
    plt.plot(X,Y, linewidth=5.0, color='b')

num_edges = int(shape_file.readline())
for _ in range(num_edges):
    edge_str = shape_file.readline()
    p1_str = edge_str[:edge_str.find(' ')]
    p2_str = edge_str[edge_str.find(' ')+1:]
    p1_x = float(eval("1.0*" + p1_str[:p1_str.find(',')]))
    p1_y = float(eval("1.0*" + p1_str[p1_str.find(',')+1:]))
    p2_x = float(eval("1.0*" + p2_str[:p2_str.find(',')]))
    p2_y = float(eval("1.0*" + p2_str[p2_str.find(',')+1:]))
    plt.plot([p1_x, p2_x], [p1_y, p2_y], linewidth=1.5, color='r')
plt.show()
