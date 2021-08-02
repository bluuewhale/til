from typing import *
from fastapi import FastAPI
from fastapi.encoders import jsonable_encoder
from pydantic import BaseModel

app = FastAPI()

@app.get("/health")
def read_health():
    return ""

@app.get("/greeting")
def read_health():
    return jsonable_encoder({"data": "Hello, World!"})
