import dataclasses
import functools
import ssl
import typing

import aiohttp
import certifi

from benchmarks.server.controller.base import RequestsCases, JSONType, Self


@typing.final
@dataclasses.dataclass
class AIOHTTPRequestCases(RequestsCases):

    base_url: str
    ssl_context: ssl.SSLContext

    path_part: typing.ClassVar[str] = "aiohttp"

    @classmethod
    def init(cls: typing.Type[Self], base_url: str) -> Self:
        return cls(
            base_url=base_url,
            ssl_context=ssl.create_default_context(cafile=certifi.where())
        )

    @functools.cached_property
    def client(self) -> aiohttp.ClientSession:
        return aiohttp.ClientSession()

    async def post_with_small_json(self, data: JSONType) -> None:
        async with self.client.post(f"{self.base_url}/post", json=data) as response:
            await response.json()
