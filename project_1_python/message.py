import json

# converts python obj to json string
def to_json(obj):
    json_str = json.dumps(obj)
    json_str += "\n"
    return json_str

# converts json string to python obj
def from_json(json_str):
    return json.loads(json_str)

# reads the next message from the socket until a newline, returns as python obj
def get_message(s):
    buffer = ""
    while True:
        data = s.recv(1024).decode()
        if data and data[-1]=='\n':
            buffer += data
            break
        elif not data:
            break
        else:
            buffer += data
    return from_json(buffer)

# encodes a pytho obj and sends it to the socket stream
def send_message(s, obj):
    s.sendall(to_json(obj).encode())

