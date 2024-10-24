import asyncio
import websockets
import json

CLIENTS = set()

async def handler(websocket):
    CLIENTS.add(websocket)
    while True:
        try:
            message = await websocket.recv()
            received_data = json.loads(message)
            print(received_data)
            if received_data['type'] == 'Ship' or received_data['type'] == 'Bullet':
                for ws in CLIENTS:
                    await ws.send(message)
            else:
                print(f'received invalid data: {message}')

        except websockets.ConnectionClosedOK:
            break

    CLIENTS.remove(websocket)

async def main():
    async with websockets.serve(handler, "0.0.0.0", 8080):
        await asyncio.get_running_loop().create_future()

asyncio.run(main())
