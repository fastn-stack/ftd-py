import json

import ftd
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
    data = {"message": "hello world", "n": 10}
    data = json.dumps(data)
    await ftd.render("foo.ftd", data)


loop = asyncio.get_event_loop()
loop.run_until_complete(render())
loop.close()
