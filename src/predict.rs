pub struct Prediction {
    pub bias: &'static str,
    pub confidence: &'static str,
    pub score: f64,
    pub expected_range: &'static str,
}

pub fn predict_mcx_gold(
    comex_pct: f64,
    usdinr_pct: f64,
    mcx_gap_pct: f64,
) -> Prediction {
    let score =
        (comex_pct * 0.5) +
        (usdinr_pct * 0.3) +
        (mcx_gap_pct * 0.2);

    if score >= 0.6 {
        Prediction {
            bias: "🔥 STRONG BULLISH",
            confidence: "HIGH",
            score,
            expected_range: "+0.8% to +1.3%",
        }
    } else if score >= 0.2 {
        Prediction {
            bias: "📈 BULLISH",
            confidence: "MEDIUM",
            score,
            expected_range: "+0.3% to +0.7%",
        }
    } else if score > -0.2 {
        Prediction {
            bias: "⚖️ NEUTRAL",
            confidence: "LOW",
            score,
            expected_range: "-0.2% to +0.2%",
        }
    } else if score > -0.6 {
        Prediction {
            bias: "📉 BEARISH",
            confidence: "MEDIUM",
            score,
            expected_range: "-0.7% to -0.3%",
        }
    } else {
        Prediction {
            bias: "❄️ STRONG BEARISH",
            confidence: "HIGH",
            score,
            expected_range: "-1.3% to -0.8%",
        }
    }
}