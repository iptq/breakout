import json

data = []

while True:
    try:
        line = input()
    except EOFError:
        break
    if not line:
        continue

    parts = line.split(" ")
    if len(parts) == 0:
        continue

    data.append(list(map(int, parts)))

obj = dict(
    rows=len(data[0]),
    cols=len(data),
    data=data,
)
print(json.dumps(obj))
