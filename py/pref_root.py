
from argparse import ArgumentParser, Namespace
from json import dumps
from pathlib import Path

def line_is_empty_or_comment(line: str) -> bool:
    return len(line) == 0 or line.isspace() or line[0] == '#'

def convert_coord(hex: str) -> int:
    stripped_of_comments, *_comments = hex.split('#')
    if stripped_of_comments.isdigit():
        return int(stripped_of_comments) & 0x7f
    else:
        return int(stripped_of_comments, 16) & 0x7f

def prepare_args() -> Namespace:
    parser = ArgumentParser()
    parser.add_argument("file")
    parser.add_argument("-o", "--output")
    parser.add_argument("-v", "--verbose", action = 'store_true')
    return parser.parse_args()

def output_json(args: Namespace, content: dict | list) -> None:
    s = dumps(content, ensure_ascii = False)
    origin = Path(args.file)
    if args.output is not None:
        target = origin.with_name(args.output)
        with target.open('w', encoding='utf-8') as o:
            o.write(s)
    else:
        new_name = Path(args.file).with_suffix(".json")
        with new_name.open('w', encoding=('utf-8')) as o:
            o.write(s)

    if args.verbose is True:
        print(s)