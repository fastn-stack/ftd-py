from ftd import fpm_build
import asyncio


async def build(file=None, base_url=None, ignore_failed=None):
    await fpm_build(file, base_url, ignore_failed)


loop = asyncio.get_event_loop()
loop.run_until_complete(build())
loop.close()
