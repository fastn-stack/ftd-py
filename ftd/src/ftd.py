import ftd_sys
import json
import asyncio


class Document:
    def __init__(self, f: str, **data):
        self.f = f
        self.data = data

    async def render(self, **data) -> str:
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        print("render", self.f, all_data)
        return await ftd_sys.render(self.f, all_data)


def parse(f: str, **data) -> Document:
    return Document(f, **data)


async def render(f: str, **data) -> str:
    d = parse(f, **data)
    return await d.render()


def render_sync(f: str, **data) -> str:
    loop = asyncio.get_event_loop()
    res = loop.run_until_complete(render(f, **data))
    loop.close()
    return res
