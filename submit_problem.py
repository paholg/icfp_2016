#!/usr/bin/python2.7

import subprocess
import sys

API_KEY = "279-4ada75226794d3f7e114f6408ad14c73"
HW_ENDPOINT = "http://2016sv.icfpcontest.org/api/hello"
SUBMIT_ENDPOINT = "http://2016sv.icfpcontest.org/api/solution/submit"

def main():
    if (len(sys.argv) != 2):
        print "Usage: ./submit_problem.py problem_file"
        exit(1)
    if (str.find(sys.argv[1], '-') == -1):
        print "Solution file format: id-infostring.txt"
        exit(1)

    solution_file = sys.argv[1]
    if (str.find(solution_file, '/') != -1):
        id_start_idx = len(solution_file) - 1 - str.index(solution_file[::-1], '/')
    else:
        id_start_idx = 0
    id_end_idx = str.index(solution_file, '-')
    problem_id = solution_file[id_start_idx+1:id_end_idx]

    cmd_list = [
        "curl",
        "--compressed",
        "-L",
        "-H", "Expect:",
        "-H", "X-API-Key: " + API_KEY,
        "-F", "problem_id=" + problem_id,
        "-F", "solution_spec=@" + solution_file,
        SUBMIT_ENDPOINT
    ]

    print subprocess.list2cmdline(cmd_list)
    out_json = subprocess.check_output(cmd_list)
    print out_json

if __name__ == "__main__":
    main()

