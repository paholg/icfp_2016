#!/usr/bin/python2.7

import hashlib
import os
import subprocess
import sys
import time

import teaminfo

solution_hash_file = 'solution_hashes.txt'

def main():
    solution_hash_text = open(solution_hash_file, 'r').read()
    if len(solution_hash_text) > 0:
        solution_hash_list = eval(solution_hash_text)
    else:
        solution_hash_list = []

    solution_names = [f for f in os.listdir('solutions') if os.path.isfile('solutions/' + f)]
    solution_files = ['solutions/' + s for s in solution_names]

    i = 0
    for solution_file in solution_files:
        i += 1
        print i, " / ", len(solution_files)
        solution_text = open(solution_file, 'r').read()
        if hash(solution_text) in solution_hash_list:
            print "Not submitting, no change"
            continue
        else:
            print "Submitting", solution_file
            submit(solution_file)
            open(solution_hash_file, "w").write(str(solution_hash_list + [hash(solution_text)]))
        time.sleep(3.6)

def submit(solution_file):
    if (str.find(solution_file, '/') != -1):
        id_start_idx = len(solution_file) - 1 - str.index(solution_file[::-1], '/')
    else:
        id_start_idx = 0
    id_end_idx = str.index(solution_file, '.')
    problem_id = solution_file[id_start_idx+1:id_end_idx]

    cmd_list = [
        "curl",
        "-s",
        "--compressed",
        "-L",
        "-H", "Expect:",
        "-H", "X-API-Key: " + teaminfo.API_KEY,
        "-F", "problem_id=" + problem_id,
        "-F", "solution_spec=@" + solution_file,
        teaminfo.PROBLEM_SUBMIT_ENDPOINT
    ]

    out_json = subprocess.check_output(cmd_list)
    print out_json

if __name__ == "__main__":
    main()
