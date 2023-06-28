FROM python:3.11

WORKDIR /app

COPY ai/pyproject.toml ./

RUN pip install --upgrade --no-cache-dir pip
RUN pip install --no-cache-dir poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-dev --no-interaction --no-ansi --no-cache

COPY ai/ ./

CMD ["poetry", "run", "uvicorn", "ai:app", "--host", "0.0.0.0", "--port", "80"]