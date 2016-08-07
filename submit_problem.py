#!/usr/bin/python2.7

import hashlib
import os
import subprocess
import sys

import teaminfo

submission_hash_file = 'submission_hashes.txt'

def main():
    submission_hash_text = open(submission_hash_file, 'r').read()
    if len(submission_hash_text) > 0:
        submission_hash_list = eval(submission_hash_text)
    else:
        submission_hash_list = []

    submission_names = [f for f in os.listdir('submissions') if os.path.isfile('submissions/' + f)]
    submission_files = ['submissions/' + s for s in submission_names]

    for submission_file in submission_files:
        submission_text = open(submission_file, 'r').read()
        if hash(submission_text) in submission_hash_list:
            continue
        else:
            print "Submitting", submission_file
            submit(submission_file)
            open(submission_hash_file, "w").write(str(submission_hash_list + [hash(submission_text)]))

def submit(solution_file):
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
        "-H", "X-API-Key: " + teaminfo.API_KEY,
        "-F", "problem_id=" + problem_id,
        "-F", "solution_spec=@" + solution_file,
        teaminfo.PROBLEM_SUBMIT_ENDPOINT
    ]

    print subprocess.list2cmdline(cmd_list)
    out_json = subprocess.check_output(cmd_list)
    print out_json

if __name__ == "__main__":
    main()
