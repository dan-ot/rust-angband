from typing import Optional, Tuple
from pref_root import convert_coord, prepare_args, output_json, line_is_empty_or_comment

def convert_line(line: str) -> Tuple[int, Tuple[int, int]]:
    _, index, row, col = line.split(':')
    return [int(index), [convert_coord(row), convert_coord(col)]]

def read_line(line: str) -> Optional[Tuple[int, Tuple[int, int]]]:
    if line_is_empty_or_comment(line):
        return None
    else:
        return convert_line(line)

args = prepare_args()

with open(args.file, encoding='utf-8') as f:
    output = []
    for line in f:
        out = read_line(line)
        if out is not None:
            idx, coords = out
            output.append({
                'index': idx,
                'coords': coords
            })
    output_json(args, output)