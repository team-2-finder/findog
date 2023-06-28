"""
Main server for image relation detection.
"""

from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def read_root():
    """
    sample function
    """
