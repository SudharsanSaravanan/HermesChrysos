from growwapi import GrowwAPI
import os
import json
from dotenv import load_dotenv

load_dotenv()

API_KEY = os.getenv("GROWW_API_KEY")
API_SECRET = os.getenv("GROWW_API_SECRET")

# Read cached symbol
with open("python/cache/mcx_gold.json", "r") as f:
    cached = json.load(f)

symbol = cached["symbol"]

access_token = GrowwAPI.get_access_token(
    api_key=API_KEY,
    secret=API_SECRET
)
groww = GrowwAPI(access_token)

quote = groww.get_quote(
    exchange=groww.EXCHANGE_MCX,
    segment=groww.SEGMENT_COMMODITY,
    trading_symbol=symbol
)

# ONLY JSON
print(json.dumps({
    "symbol": symbol,
    "ltp": quote.get("last_price"),
    "prev_close": quote.get("ohlc", {}).get("close")
}))
