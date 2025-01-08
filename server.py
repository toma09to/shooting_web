import asyncio
import websockets
import json

CLIENTS = set()
is_started = set()
join_list = [False, False, False, False]
ready_list = [False, False, False, False]

async def handler(websocket):
    global is_started

    ranking = []
    CLIENTS.add(websocket)
    while True:
        try:
            message = await websocket.recv()
            received_data = json.loads(message)
            # print(received_data, flush=True)
            
            if websocket in is_started:
                if received_data['type'] == 'Ship' \
                    and received_data['data']['lives'] <= 0 \
                    and received_data['data']['id'] not in ranking:
                    ranking.append(received_data['data']['id'])
                if received_data['type'] == 'Ship' or received_data['type'] == 'Bullet':
                    for ws in CLIENTS:
                        await ws.send(message)
                else:
                    print(f'received invalid data: {message}')

                if len(ranking) >= sum(join_list) - 1:
                    ranking.append(sum(join_list) - sum(ranking))
                    ranking.reverse()
                    for ws in CLIENTS:
                        await ws.send(json.dumps({
                            'type': 'End',
                            'data': {
                                'ranking': ranking
                            }
                        }))
                    websocket.close()
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
                        for ws in CLIENTS:
                            await ws.send(json.dumps({
                                'type': 'Start',
                                'data': {}
                            }))
                elif received_data['type'] == 'Started':
                    for ws in CLIENTS:
                        is_started.add(ws)
                else:
                    print(f'received invalid data: {message}')

        except websockets.ConnectionClosedOK:
            if 'ship_id' in locals():
                join_list[ship_id] = False
                is_started.remove(websocket)
            break

    CLIENTS.remove(websocket)

async def main():
    async with websockets.serve(handler, "0.0.0.0", 8080):
        await asyncio.get_running_loop().create_future()

asyncio.run(main())
