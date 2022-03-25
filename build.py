from ftd import fpm_build
import asyncio

async def build():
    await fpm_build()
    print("done")

loop = asyncio.get_event_loop()
loop.run_until_complete(build())
loop.close()