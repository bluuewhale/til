import json 

def write_json(obj, path):
    with open(path, 'w', encoding='utf-8') as f:
        f.write(json.dumps(obj, ensure_ascii=False, indent='\t'))

def read_json(path):
    with open(path, 'r', encoding="utf-8") as f:
        return json.load(f)