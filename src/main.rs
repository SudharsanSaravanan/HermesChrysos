mod config;
mod fetch;
mod predict;

#[tokio::main]
async fn main() {
    config::load();

    println!("\nHermesChrysos — Gold Outlook");
    println!("⚠︎ Educational purpose only — not financial advice\n");

    // ── COMEX Gold ──
    let (comex_price, comex_prev, comex_change_pct) =
        fetch::gold::get_gold().await;

    // ── USD → INR ──
    let usdinr = fetch::fx::get_usdinr().await;

    // TEMP until previous close is stored
    let usdinr_prev = usdinr - 0.20;
    let usdinr_pct =
        ((usdinr - usdinr_prev) / usdinr_prev) * 100.0;

    // ── MCX Gold ──
    let (symbol, mcx_ltp, mcx_prev) =
        fetch::mcx::get_mcx_gold();

    let mcx_gap_pct =
        ((mcx_ltp - mcx_prev) / mcx_prev) * 100.0;

    // ── Prediction ──
    let prediction = predict::predict_mcx_gold(
        comex_change_pct,
        usdinr_pct,
        mcx_gap_pct,
    );

    // ── PRINT MARKET DATA ──
    println!("\n🪙 COMEX Gold (XAU/USD)");
    println!("   Price        : ${:.2}", comex_price);
    println!("   Prev Close   : ${:.2}", comex_prev);
    println!("   Change       : {:.2}%", comex_change_pct);

    println!("\n💱 USD → INR");
    println!("   Rate         : ₹{:.2}", usdinr);
    println!("   Change       : {:.2}%", usdinr_pct);

    println!("\nMCX Gold Futures");
    println!("   Contract     : {}", symbol);
    println!("   LTP          : ₹{:.2}", mcx_ltp);
    println!("   Prev Close   : ₹{:.2}", mcx_prev);
    println!("   Gap          : {:.2}%", mcx_gap_pct);

    // ── EDUCATIONAL EXPLANATION ──
    println!("\n1. HOW THIS PREDICTION IS CALCULATED");
    println!("   MCX Gold is influenced by:");
    println!("   1. Global Gold movement (COMEX)");
    println!("   2. USD → INR currency movement");
    println!("   3. Local MCX futures positioning");

    println!("\nFORMULA USED");
    println!("   Prediction Score =");
    println!("     (COMEX % × 0.5)");
    println!("   + (USDINR % × 0.3)");
    println!("   + (MCX Gap % × 0.2)");

    println!("\nVALUES FROM APIs");
    println!(
        "   COMEX Contribution : {:.2} × 0.5 = {:.2}",
        comex_change_pct,
        comex_change_pct * 0.5
    );
    println!(
        "   USDINR Contribution: {:.2} × 0.3 = {:.2}",
        usdinr_pct,
        usdinr_pct * 0.3
    );
    println!(
        "   MCX Contribution   : {:.2} × 0.2 = {:.2}",
        mcx_gap_pct,
        mcx_gap_pct * 0.2
    );

    // ── FINAL PREDICTION ──
    println!("\nGOLD PREDICTION RESULT");
    println!("   Bias         : {}", prediction.bias);
    println!("   Confidence   : {}", prediction.confidence);
    println!("   Score        : {:.2}", prediction.score);
    println!("   Expected     : {}", prediction.expected_range);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}