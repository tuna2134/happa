from happa import Server, WebsocketStream
from threading import Thread


def listen(ws: WebsocketStream):
    ws.send(b"ok")
    msg = ws.recv()
    if msg.is_text():
        print(msg.text())

def listener(ws: WebsocketStream):
    thread = Thread(target=listen, args=(ws,), daemon=True)
    thread.start()


s = Server(listener)
s.start("0.0.0.0:8080")