from typing import Iterable, Optional, Tuple
from pref_root import convert_coord, prepare_args, output_json, line_is_empty_or_comment

def interpret_field(field: str) -> Tuple[str, str]:
    _, selector, target, *_ = field.split(' ')
    return [selector.lstrip('$'), target.strip().rstrip(']')]

class Filter:
    def __init__(self, op: str, fields: list[Tuple[str, str]]):
        self.op = op
        self.fields = fields

    def from_str(in_str: str) -> 'Filter':
        combo_or_op, *fieldset = in_str.split('[')[1:]
        match combo_or_op.strip().lower():
            case 'and':
                return Filter('and', [interpret_field(f) for f in fieldset])
            case _:
                return Filter('and', [interpret_field(f) for f in [combo_or_op] + fieldset])

class Value:
    def __init__(self, name: str, coords: Tuple[int, int]):
        self.name = name
        self.coords = coords

    def from_str(str: Iterable[str]) -> 'Value':
        return Value(str[0], [convert_coord(str[1]), convert_coord(str[2])])

def convert_line(line: str) -> Optional[Filter | Value]:
    line_type, *rest = line.split(':')
    match line_type:
        case "?":
            return Filter.from_str(rest[0])
        case "monster":
            return Value.from_str(rest)
        case _:
            return None

def read_line(num: int, line: str) -> Optional[Filter | Value]:
    if line_is_empty_or_comment(line):
        return None
    else:
        converted = convert_line(line)
        if converted is None:
            print("Failed to convert line {}: [{}]".format(num, line))
        return converted

args = prepare_args()

with open(args.file) as f:
    output = []
    last_seen = None
    for num, line in enumerate(f):
        match read_line(num, line):
            case None:
                pass
            case Filter() as filter:
                match last_seen:
                    case Filter() as ls_f:
                        print("Line {} out-of-sequence Filter.".format(num - 1))
                last_seen = filter
            case Value() as value:
                match last_seen:
                    case Filter() as ls_f:
                        output.append({
                            'apply': value.__dict__,
                            'when': ls_f.__dict__
                        })
                    case Value() as ls_v:
                        print("Line {} out-of-sequence VAlue.".format(num - 1))
                last_seen = value
    
    output_json(args, output)