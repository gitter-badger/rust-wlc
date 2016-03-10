#!/usr/bin/env python3

## Utility functions used by the other modules
import sys

RED_TEXT = '\033[91m'
GREEN_TEXT = '\033[92m'
YELLOW_TEXT = '\033[93m'
END_TEXT = '\033[0m'


def log(string, log_type):
    log_type = log_type.lower().strip()
    if log_type == "success":
        color = GREEN_TEXT + "Success: "
    elif log_type == "error":
        color = RED_TEXT + "Error: "
    elif log_type == "warning":
        color = YELLOW_TEXT + "Warning: "
    else:
        raise ValueError("Bad log type {}".format(log_type))
    sys.stderr.write(color + string + END_TEXT +  "\n")

def delete_comments(lines: list) -> list:
    """Deletes all the stuff that appears in comments.
    If the comment is multiline, the lines are just not included"""
    result = []
    in_comment = False
    for line in lines:
        if in_comment:
            end = line.find("*/")
            if end == -1:
                continue
            result.append(line[end + len("*/"):])
            in_comment = False
            continue
        comment_start = line.find("//")
        if comment_start == -1:
            result.append(line)
        else:
            result.append(line[:comment_start])
    return result
