from ftd_sys import fpm_build
import asyncio


async def build(root=None, file=None, base_url=None, ignore_failed=None):
    await fpm_build(root, file, base_url, ignore_failed)


loop = asyncio.get_event_loop()
loop.run_until_complete(build(root="../sample"))
loop.close()
