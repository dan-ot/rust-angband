from typing import Iterable, Optional, Tuple
from pref_root import convert_coord, prepare_args, output_json, line_is_empty_or_comment

class Effect():
    def __init__(self, ts: list[str], d: str, coords: Tuple[int, int]) -> 'Effect':
        self.types = ts
        self.direction = d
        self.coords = coords

class Feature():
    def __init__(self, name: str, lighting: str, coords: Tuple[int, int]) -> 'Feature':
        self.name = name
        self.lighting = lighting
        self.coords = coords

class Trap():
    def __init__(self,name: str, lighting: str, coords: Tuple[int, int]):
        self.name = name
        self.lighting = lighting
        self.coords = coords

class GameObject():
    def __init__(self, tval: str, sval: str, coords: Tuple[int, int]):
        self.tval = tval
        self.sval = sval
        self.coords = coords

class Monster():
    def __init__(self, name: str, coords: Tuple[int, int]):
        self.name = name
        self.coords = coords

Parseable = Effect | Feature | Trap | GameObject | Monster

def convert_effect(rest: Iterable[str]) -> Effect:
    effects, direction, row, col = rest
    effect_list = effects.split("|")
    return Effect(effect_list, direction, [convert_coord(row), convert_coord(col)])

def convert_feature(rest: Iterable[str]) -> Feature:
    name, lighting, row, col = rest
    return Feature(name, lighting, [convert_coord(row), convert_coord(col)])

def convert_trap(rest: Iterable[str]) -> Trap:
    name, lighting, row, col = rest
    return Trap(name, lighting, [convert_coord(row), convert_coord(col)])

def convert_object(rest: Iterable[str]) -> GameObject:
    tval, sval, row, col = rest
    return GameObject(tval, sval, [convert_coord(row), convert_coord(col)])

def convert_monster(rest: Iterable[str]) -> Monster:
    name, row, col = rest
    return Monster(name, [convert_coord(row), convert_coord(col)])

def convert_line(line: str) -> Parseable:
    line_type, *rest = line.split(':')
    match line_type:
        case "GF":
            return convert_effect(rest)
        case "feat":
            return convert_feature(rest)
        case "trap":
            return convert_trap(rest)
        case "object":
            return convert_object(rest)
        case "monster":
            return convert_monster(rest)

def read_line(line: str) -> Optional[Parseable]:
    if line_is_empty_or_comment(line):
        return None
    else:
        return convert_line(line)

args = prepare_args()
with open(args.file, encoding='utf-8') as f:
    output = {
        "effects": {},
        "features": {},
        "traps": {},
        "objects": {},
        "monsters": [],
    }
    for line in enumerate(f):
        out = read_line(line)
        match out:
            case Effect():
                for t in out.types:
                    if t in output['effects']:
                        output['effects'][t].append({'direction': out.direction, 'coords': out.coords})
                    else:
                        output['effects'][t] = [{'direction': out.direction, 'coords': out.coords}]
            case Feature():
                if out.name in output['features']:
                    output['features'][out.name].append({'lighting': out.lighting, 'coords': out.coords})
                else:
                    output['features'][out.name] = [{'lighting': out.lighting, 'coords': out.coords}]
            case Trap():
                if out.name in output['traps']:
                    output['traps'][out.name].append({'lighting': out.lighting, 'coords': out.coords})
                else:
                    output['traps'][out.name] = [{'lighting': out.lighting, 'coords': out.coords}]
            case GameObject():
                if out.tval in output['objects']:
                    output['objects'][out.tval].append({'sval': out.sval, 'coords': out.coords})
                else:
                    output['objects'][out.tval] = [{'sval': out.sval, 'coords': out.coords}]
            case Monster():
                output['monsters'].append(out.__dict__)
            case None:
                pass

    output_json(args, output)
