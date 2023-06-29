# pylint: disable=too-few-public-methods

"""
This file defines the database models.
"""

import datetime
from typing import Optional

from sqlalchemy.ext.asyncio import AsyncAttrs
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column


class Base(AsyncAttrs, DeclarativeBase):
    """
    Base class for all models
    """


class Dogs(Base):
    """
    Dogs model
    """

    __tablename__ = "dogs"
    desertion_no: Mapped[str] = mapped_column(primary_key=True)
    filename: Mapped[str]
    image_path: Mapped[Optional[str]]
    happen_dt: Mapped[datetime.datetime]
    kind_cd: Mapped[str]
    color_cd: Mapped[str]
    age: Mapped[str]
    weight: Mapped[str]
    sex_cd: Mapped[str]
    neuter_yn: Mapped[str]
    care_nm: Mapped[str]
    care_tel: Mapped[str]
    care_addr: Mapped[str]
    charge_nm: Mapped[Optional[str]]
    officetel: Mapped[str]
    notice_comment: Mapped[Optional[str]]
