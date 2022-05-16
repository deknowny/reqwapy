import fastapi

from benchmarks.server.controller.aiohttp_impl import AIOHTTPRequestCases
from benchmarks.server.controller.reqwapy_impl import ReqwapyRequestCases


app = fastapi.FastAPI()
base_url = "http://localhost:8000"

reqwapy_cases = ReqwapyRequestCases.init(base_url)
aiohttp_cases = AIOHTTPRequestCases.init(base_url)

reqwapy_cases.register(app)
aiohttp_cases.register(app)
