import asyncio
import ssl

import pytest
import certifi
import reqwapy.core
import aiohttp

sslcontext = ssl.create_default_context(cafile=certifi.where())

BASE_URL = "http://0.0.0.0:80"
CONCURRENTS = 100


async def inner_aiohttp():
    client = aiohttp.ClientSession()
    try:
        async with client.post(f"{BASE_URL}/post", ssl=sslcontext) as response:
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
    response = await client.request_json("POST", "/post")


def test_reqwapy(benchmark):
    benchmark(lambda: asyncio.run(
        asyncio.wait([
                inner_reqwapy() for i in range(CONCURRENTS)
            ])
        )
    )
