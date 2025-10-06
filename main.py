import asyncio
import os
import rust_src

from dotenv import load_dotenv


load_dotenv()
async def main() -> None:
    token: str = os.getenv("DISCORD_TOKEN", "")
    await rust_src.start_bot_py(token)
    
if __name__ == "__main__":
    asyncio.run(main=main())