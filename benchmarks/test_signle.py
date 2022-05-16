import asyncio
import ssl
import pathlib
import json

import pytest
import certifi
import reqwapy.core
import aiohttp

sslcontext = ssl.create_default_context(cafile=certifi.where())

BASE_URL = "http://0.0.0.0:80"
CONCURRENTS = 1
ITERS = 50

with open(pathlib.Path("benchmarks", "small.json")) as filejson:
    request_json = json.load(filejson)


async def inner_aiohttp():
    client = aiohttp.ClientSession()
    try:
        for i in range(ITERS):
            async with client.post(f"{BASE_URL}/ping/", json=request_json) as response:
                body = await response.json()
    finally:
        await client.close()


def test_aiohttp(benchmark):
    benchmark(lambda: asyncio.run(
        asyncio.wait([
                inner_aiohttp() for i in range(CONCURRENTS)
            ])
        )
    )


async def inner_reqwapy():
    client = reqwapy.core.client.Client(base_url=BASE_URL)
    for i in range(ITERS):
        response = await client.request_json("POST", "/ping/", json=request_json,)


def test_reqwapy(benchmark):
    benchmark(lambda: asyncio.run(
        asyncio.wait([
                inner_reqwapy() for i in range(CONCURRENTS)
            ])
        )
    )
