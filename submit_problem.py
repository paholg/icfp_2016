#!/usr/bin/python2.7

import datetime
import hashlib
import json
import os
import subprocess
import sys
import time

import teaminfo

solution_hash_file = 'solution_hashes.txt'

def main():
    solution_hash_text = open(solution_hash_file, 'r').read()
    if len(solution_hash_text) > 0:
        solution_hash_dict = eval(solution_hash_text)
    else:
        solution_hash_dict = {}

    solution_names = [f for f in os.listdir('solutions') if os.path.isfile('solutions/' + f)]
    solution_files = ['solutions/' + s for s in solution_names]

    solution_hashes = [data[0] for data in solution_hash_dict.values()]

    i = 0
    for solution_file in solution_files:
        i += 1
        sys.stdout.write("(" + str(i) + "/" + str(len(solution_files)) + ") ")
        solution_text = open(solution_file, 'r').read()
        if str(hash(solution_text)) in solution_hashes:
            sys.stdout.write("Not submitting, no change.\n")
            continue
        else:
            sys.stdout.write("Submitting " + str(solution_file) + ": ")
            (solution_id, result_bool, result_msg) = submit(solution_file)
            result = str(result_bool)

            data_str_list = [str(hash(solution_text)), result, result_msg, str(datetime.datetime.now())]
            solution_hash_dict[solution_id] = data_str_list
            open(solution_hash_file, "w").write(str(solution_hash_dict))
            sys.stdout.write(result_msg + '\n')
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

    submit_return_json = subprocess.check_output(cmd_list)
    submit_return_dict = json.loads(submit_return_json)
    if (submit_return_dict[u'ok'] == 0):
        return (problem_id, False, submit_return_dict[u'error'])
    else:
        return (problem_id, True, submit_return_dict[u'resemblance'])

if __name__ == "__main__":
    main()
