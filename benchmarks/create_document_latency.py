#!/usr/bin/env python3
import asyncio, json, ssl, time, websockets, random, statistics, sys

# === Configuration ===
TOKEN=""
URL  = f"wss://olta-vm.load.network/ws/J1TjLzI8U8tXbd4SGm2AkdHUcnMSbwMNwgSXdJzkvpo?token={TOKEN}"
RUNS = 3
COLL = "vertices"

def make_doc(i):
    return {
        "_id": i,
        "_creator": "user1",
        "type": "vertices",
        "x": f"{random.randint(0,1000)}n",
        "y": f"{random.randint(0,1000)}n",
        "z": f"{random.randint(0,1000)}n",
        "lineColor": "16711680n",
        "vertexColor": "65280n",
        "cameraX": "0n",
        "cameraY": "0n",
        "cameraZ": "10n",
    }

async def send_once(ws, payload):
    """Send one message and measure round-trip in microseconds."""
    t0 = time.perf_counter_ns()
    await ws.send(json.dumps(payload))
    try:
        await asyncio.wait_for(ws.recv(), timeout=3)
    except asyncio.TimeoutError:
        pass
    t1 = time.perf_counter_ns()
    return (t1 - t0) / 1000.0  # convert ns → µs

async def main():
    sslctx = ssl.create_default_context()
    print(f"Connecting to {URL}")
    print("Instruction: CreateDocument\n")
    latencies = []

    async with websockets.connect(URL, ssl=sslctx, ping_interval=None, max_size=None) as ws:
        for i in range(RUNS + 1):
            doc = make_doc(i)
            payload = {"CreateDocument": {"collection_name": COLL, "document": doc}}
            rtt = await send_once(ws, payload)
            if i > 0:
                latencies.append(rtt)
                avg = statistics.mean(latencies)
                jitter = statistics.pstdev(latencies) if len(latencies) > 1 else 0.0
                fps = 1_000_000 / rtt if rtt > 0 else 0.0
                sys.stdout.write(
                    f"\rPing: {rtt:9.2f} µs | Avg: {avg:9.2f} µs | "
                    f"Jitter: {jitter:9.2f} µs | FPS: {fps:7.1f}     "
                )
                sys.stdout.flush()
            await asyncio.sleep(0.3)

    print("\n\n--- Final Stats ---")
    print(f"Samples: {len(latencies)}")
    print(f"Average: {statistics.mean(latencies):.2f} µs")
    print(f"Min: {min(latencies):.2f} µs")
    print(f"Max: {max(latencies):.2f} µs")
    print(f"Jitter (stdev): {statistics.pstdev(latencies):.2f} µs")

if __name__ == "__main__":
    asyncio.run(main())
