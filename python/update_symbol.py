from growwapi import GrowwAPI
import pandas as pd
import io
import requests
import os
import json
from dotenv import load_dotenv
from datetime import datetime

load_dotenv()

API_KEY = os.getenv("GROWW_API_KEY")
API_SECRET = os.getenv("GROWW_API_SECRET")

access_token = GrowwAPI.get_access_token(
    api_key=API_KEY,
    secret=API_SECRET
)
groww = GrowwAPI(access_token)

url = "https://growwapi-assets.groww.in/instruments/instrument.csv"
raw = requests.get(url, timeout=15).content

df = pd.read_csv(io.StringIO(raw.decode("utf-8")), low_memory=False)

fut = df[
    (df["exchange"] == "MCX") &
    (df["segment"] == "COMMODITY") &
    (df["trading_symbol"].str.startswith("GOLDM")) &
    (df["trading_symbol"].str.endswith("FUT"))
].copy()

fut["expiry"] = pd.to_datetime(fut["expiry_date"])
active = fut[fut["expiry"] >= datetime.now()].sort_values("expiry")

symbol = active.iloc[0]["trading_symbol"]

os.makedirs("python/cache", exist_ok=True)

with open("python/cache/mcx_gold.json", "w") as f:
    json.dump(
        {
            "symbol": symbol,
            "updated_at": datetime.now().isoformat()
        },
        f
    )
