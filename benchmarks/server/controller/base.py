from __future__ import annotations

import abc
import dataclasses
import json
import pathlib

import typing

import fastapi

JSONType = typing.Dict[str, typing.Any]
Self = typing.TypeVar("Self")


def load_json(name: str) -> dict:
    full_path = pathlib.Path("benchmarks", "server", "data", f"{name}.json")
    return json.load(full_path.open("rb"))


@dataclasses.dataclass
class RequestsCases(abc.ABC):

    path_part: typing.ClassVar[str]

    @classmethod
    @abc.abstractmethod
    def init(cls: typing.Type[Self], base_url: str) -> Self:
        pass

    @abc.abstractmethod
    async def post_with_small_json(self, data: JSONType) -> None:
        pass

    def register(self, app: fastapi.FastAPI) -> None:

        small_json = load_json("small")

        @app.get(f"/bench/{self.path_part}/post_with_small_json")
        async def handler():
            await self.post_with_small_json(small_json)

        @app.post(f"/ping/")
        async def handler():
            return small_json
