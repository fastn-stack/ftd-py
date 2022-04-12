import ftd
import asyncio


async def render():
    d = ftd.parse("foo.ftd", message="FTD renderer says hello world", n=10)
    return await d.render(n=12)


loop = asyncio.get_event_loop()
res = loop.run_until_complete(render())
print(res)
loop.close()
