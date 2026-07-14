FROM python:3.12-slim AS builder

WORKDIR /app
COPY pyproject.toml .
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir --prefix=/install .

COPY . .
RUN pip install --no-cache-dir --prefix=/install .

FROM python:3.12-slim AS runtime

RUN groupadd -r synapsis && useradd -r -g synapsis -u 1000 synapsis

WORKDIR /app
COPY --from=builder /install /usr/local
COPY --from=builder /app /app

RUN chown -R synapsis:synapsis /app
USER synapsis

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD python -c "import httpx; httpx.get('http://localhost:8000/api/v1/health')" || exit 1

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
