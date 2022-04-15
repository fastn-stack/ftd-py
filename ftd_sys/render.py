import json

import ftd_sys
import asyncio

# class Document:
#     async def __init__(self, f, **data):
#         self.f = f
#         self.data = data
#
#     async def render(self, **data):
#         data += self.data
#         data = json.dumps(data)
#         await ftd.render(self.f, data)
#
#
# async def parse(f, **data):
#     return Document(f, **data)
#
#
# async def hello():
#     d = await parse("foo.ftd", message="hello world", n=10)
#     await d.render()


async def render():
    data = {"asd": "hello world!!!", "n": 10, "kv2": {"key": "asd2", "value": "something2"}}
    data = json.dumps(data)
    return await ftd_sys.render("foo", data, "sample")


loop = asyncio.get_event_loop()
res = loop.run_until_complete(render())
print(res)
loop.close()
