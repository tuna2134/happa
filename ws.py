from websockets import connect

import asyncio


async def main():
    async with connect("ws://localhost:8081") as ws:
        print("Connected")
        print(await ws.recv())
        await ws.send(b"hello")


asyncio.run(main())