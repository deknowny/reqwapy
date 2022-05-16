import dataclasses
import typing

import reqwapy

from benchmarks.server.controller.base import RequestsCases, JSONType, Self


@typing.final
@dataclasses.dataclass
class ReqwapyRequestCases(RequestsCases):

    client: reqwapy.core.client.Client

    path_part: typing.ClassVar[str] = "reqwapy"

    @classmethod
    def init(cls: typing.Type[Self], base_url: str) -> Self:
        client = reqwapy.core.client.Client(base_url=base_url)
        return cls(client=client)

    async def post_with_small_json(self, data: JSONType) -> None:
        await self.client.request_json("POST", "/post", json=data)
