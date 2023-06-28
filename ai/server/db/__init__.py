# pylint: disable=invalid-name, broad-exception-raised, broad-exception-caught

"""
Database module

This connects to database and can be used in other modules to access database

example:
    >>> from data.db import Database
    >>> await Database.init()
    >>> async with Database.async_session() as session:
    ...     stmt = select(models.Restaurant).where(models.Restaurant.id == 1)
    ...     res = (await session.execute(stmt)).scalar()
    ...     print(res)
"""

import asyncio

from sqlalchemy.ext.asyncio import (
    AsyncEngine,
    AsyncSession,
    async_sessionmaker,
    create_async_engine,
)

from ..env import get_env


class Database:
    """
    Database class
    """

    engine: AsyncEngine | None
    async_session: async_sessionmaker[AsyncSession] | None

    @classmethod
    async def init(cls) -> None:
        """
        This initializes the database

        This should be called before using the database
        """
        if hasattr(cls, "engine") and cls.engine is not None:
            return

        username = get_env("SQL_USER")
        password = get_env("SQL_PASSWORD")
        url = get_env("SQL_HOST")
        db = get_env("SQL_DB")

        for _ in range(int(get_env("SQL_RETRY"))):
            try:
                print("Connecting to database...")
                engine = create_async_engine(
                    f"postgresql+asyncpg://{username}:{password}@{url}/{db}", echo=True
                )
                break
            except KeyboardInterrupt as err:
                raise KeyboardInterrupt from err
            except Exception:
                print("Failed to connect to database, retrying...")
                await asyncio.sleep(5)
        else:
            raise Exception("Unable to connect to database")
        cls.engine = engine
        cls.async_session = async_sessionmaker(cls.engine, expire_on_commit=False)

    @classmethod
    async def deinit(cls) -> None:
        """
        This deinitializes the database

        """
        if hasattr(cls, "engine") and cls.engine is not None:
            await cls.engine.dispose()
