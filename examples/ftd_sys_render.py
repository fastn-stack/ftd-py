import json

import ftd_sys
import asyncio


async def render():
    data = {"asd": "hello world!!!", "n": 14}
    data = json.dumps(data)
    return await ftd_sys.render("../sample", "/foo/", "/", data)


loop = asyncio.get_event_loop()
res = loop.run_until_complete(render())
print(res)
loop.close()
