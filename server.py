import asyncio
import websockets
import json

CLIENTS = set()
is_started = False
join_list = [False, False, False, False]
ready_list = [False, False, False, False]
ranking = []

async def handler(websocket):
    is_started = False
    CLIENTS.add(websocket)
    while True:
        try:
            message = await websocket.recv()
            received_data = json.loads(message)
            print(received_data, flush=True)
            
            if is_started:
                if received_data['type'] == 'Ship' or received_data['type'] == 'Bullet':
                    if received_data['data']['lives'] <= 0:
                        ranking.append(received_data['data']['id'])
                    for ws in CLIENTS:
                        await ws.send(message)
                else:
                    print(f'received invalid data: {message}')

                if len(ranking) >= sum(join_list) - 1:
                    ranking.append(6 - sum(ranking))
                    ranking.reverse()
                    for ws in CLIENTS:
                        await ws.send(json.dumps({
                            'type': 'End',
                            'data': {
                                'ranking': ranking
                            }
                        }))
            else:
                if received_data['type'] == 'JoinReq':
                    for i in range(4):
                        if not join_list[i]:
                            join_list[i] = True
                            ship_id = i
                            break
                    else:
                        await websocket.send(json.dumps({
                            'type': 'Full',
                            'data': {}
                        }))
                        break

                    await websocket.send(json.dumps({
                        'type': 'Join',
                        'data': {
                            'token': received_data['data']['token'],
                            'id': ship_id
                        }
                    }))
                elif received_data['type'] == 'Joined':
                    for ws in CLIENTS:
                        await ws.send(json.dumps({
                            'type': 'Entry',
                            'data': {
                                'list': join_list
                            }
                        }))
                elif received_data['type'] == 'Ready':
                    ready_list[ship_id] = True
                    for ws in CLIENTS:
                        await ws.send(json.dumps({
                            'type': 'Ready',
                            'data': {
                                'id': ship_id
                            }
                        }))

                    # If all players are ready, start a game
                    for i in range(4):
                        if join_list[i] == True and ready_list[i] == False:
                            break
                    else:
                        await asyncio.sleep(1)
                        is_started = True
                        for ws in CLIENTS:
                            await ws.send(json.dumps({
                                'type': 'Start',
                                'data': {}
                            }))
                elif received_data['type'] == 'Started':
                    is_started = True
                else:
                    print(f'received invalid data: {message}')

        except websockets.ConnectionClosedOK:
            if 'ship_id' in locals():
                join_list[ship_id] = False
            break

    CLIENTS.remove(websocket)

async def main():
    async with websockets.serve(handler, "0.0.0.0", 8080):
        await asyncio.get_running_loop().create_future()

asyncio.run(main())
