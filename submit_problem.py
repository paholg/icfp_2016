#!/usr/bin/python2.7
import subprocess
import sys

API_KEY = "279-4ada75226794d3f7e114f6408ad14c73"
HW_ENDPOINT = "http://2016sv.icfpcontest.org/api/hello"
SUBMIT_ENDPOINT = "http://2016sv.icfpcontest.org/api/solution/submit"

def main(problem_id, solution_file):
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
    if (len(sys.argv) != 3):
        print "Usage: ./submit_problem PROBLEM_ID solution_file"
        exit(1)
    main(sys.argv[1], sys.argv[2])

