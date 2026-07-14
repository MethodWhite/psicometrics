"""Tests for API endpoints."""

import pytest
from httpx import AsyncClient, ASGITransport
from app.main import app


@pytest.fixture
async def client():
    transport = ASGITransport(app=app)
    async with AsyncClient(transport=transport, base_url="http://test") as ac:
        yield ac


@pytest.mark.asyncio
async def test_health(client):
    response = await client.get("/api/v1/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "ok"


@pytest.mark.asyncio
async def test_list_tests(client):
    response = await client.get("/api/v1/tests")
    assert response.status_code == 200
    tests = response.json()
    assert len(tests) == 4
    test_types = [t["test_type"] for t in tests]
    assert "big_five" in test_types
    assert "mbti" in test_types
    assert "enneagram" in test_types
    assert "disc" in test_types


@pytest.mark.asyncio
async def test_get_big_five(client):
    response = await client.get("/api/v1/tests/big_five?lang=es")
    assert response.status_code == 200
    data = response.json()
    assert data["test_type"] == "big_five"
    assert len(data["questions"]) == 120


@pytest.mark.asyncio
async def test_get_mbti(client):
    response = await client.get("/api/v1/tests/mbti?lang=en")
    assert response.status_code == 200
    data = response.json()
    assert data["test_type"] == "mbti"
    assert len(data["questions"]) == 72


@pytest.mark.asyncio
async def test_get_enneagram(client):
    response = await client.get("/api/v1/tests/enneagram")
    assert response.status_code == 200
    data = response.json()
    assert data["test_type"] == "enneagram"
    assert len(data["questions"]) == 81


@pytest.mark.asyncio
async def test_get_disc(client):
    response = await client.get("/api/v1/tests/disc")
    assert response.status_code == 200
    data = response.json()
    assert data["test_type"] == "disc"
    assert len(data["questions"]) == 28


@pytest.mark.asyncio
async def test_invalid_test_type(client):
    response = await client.get("/api/v1/tests/invalid")
    assert response.status_code == 404


@pytest.mark.asyncio
async def test_submit_big_five(client):
    answers = [{"question_id": i, "value": 3} for i in range(1, 121)]
    response = await client.post(
        "/api/v1/tests/big_five/submit",
        json={"answers": answers, "language": "es"},
    )
    assert response.status_code == 200
    result = response.json()
    assert "scores" in result
    assert "profile_summary" in result


@pytest.mark.asyncio
async def test_submit_mbti(client):
    answers = [{"question_id": i, "value": "a" if i % 2 == 0 else "b"} for i in range(1, 73)]
    response = await client.post(
        "/api/v1/tests/mbti/submit",
        json={"answers": answers, "language": "en"},
    )
    assert response.status_code == 200
    result = response.json()
    assert "type_code" in result
    assert len(result["type_code"]) == 4
