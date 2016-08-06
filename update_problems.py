#!/usr/bin/python2.7

import json
import os
import subprocess
import sys
import time

import teaminfo

def main():
    list_snapshots_cmd_list = [
        "curl",
        "--compressed",
        "-L",
        "-H", "Expect:",
        "-H", "X-API-Key: " + teaminfo.API_KEY,
        teaminfo.LIST_SNAPSHOTS_ENDPOINT
    ]

    snapshot_list_json = subprocess.check_output(list_snapshots_cmd_list)
    snapshot_list_dict = json.loads(snapshot_list_json)
    snapshots = [s[u'snapshot_time'] for s in snapshot_list_dict[u'snapshots']]
    newest_snapshot_hash = [d[u'snapshot_hash'] for d in snapshot_list_dict[u'snapshots'] if d[u'snapshot_time'] == max(snapshots)][0]

    time.sleep(1)

    snapshot_blob_cmd_list = [
        "curl",
        "--compressed",
        "-L",
        "-H", "Expect:",
        "-H", "X-API-Key: " + teaminfo.API_KEY,
        teaminfo.GET_BLOB_ENDPOINT + newest_snapshot_hash
    ]

    snapshot_blob_json = subprocess.check_output(snapshot_blob_cmd_list)
    snapshot_blob_dict = json.loads(snapshot_blob_json)

    problem_spec_hashes = [problem[u'problem_spec_hash'] for problem in snapshot_blob_dict[u'problems']]
    problem_ids = [problem[u'problem_id'] for problem in snapshot_blob_dict[u'problems']]

    subprocess.call(["mkdir", "problems"])
    for p in range(len(problem_spec_hashes)):
        print "problem " + str(p) + " of " + str(len(problem_spec_hashes))
        p_file_name = "problems/" + str(problem_ids[p]) + ".txt"
        if os.path.isfile(p_file_name):
            continue
        else:
            time.sleep(1)
            problem_file_cmd_list = [
                "curl",
                "--compressed",
                "-L",
                "-H", "Expect:",
                "-H", "X-API-Key: " + teaminfo.API_KEY,
                teaminfo.GET_BLOB_ENDPOINT + problem_spec_hashes[p]
            ]
            problem_file_blob_txt = subprocess.check_output(problem_file_cmd_list)
            if (str.find("Error", problem_file_blob_txt) != -1):
                print "Got an error, on to the next one."
                continue
            p_file = open(p_file_name, "w")
            p_file.write(problem_file_blob_txt)
            p_file.close()
            print "Wrote ", p_file_name            
        

if __name__ == "__main__":
    main()
