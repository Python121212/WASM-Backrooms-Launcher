use wasm_bindgen::prelude::*;
use aes::Aes256;
use cbc::Decryptor;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};

// ブラウザのconsole.logと連携
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 初期化時にブラウザのコンソールへ出力を繋ぐ
#[wasm_bindgen(start)]
pub fn init_engine() {
    // パニック時のエラー追跡をブラウザ側で有効化
    console_error_panic_hook::set_once();
    log("[WNSV RUST] 仮想化コアエンジンが正常にロードされました。");
}

// Steamのデポチャンク（AES-256-CBC暗号化データ）をWasm内で超高速復号する関数
#[wasm_bindgen]
pub fn decrypt_steam_chunk(
    key_bytes: &[u8],       // Steamから取得したデポ復号キー (32 bytes)
    iv_bytes: &[u8],        // CBCモード用初期化ベクトル (16 bytes)
    encrypted_data: &[u8]   // ダウンロードした暗号データ
) -> Result<Vec<u8>, JsValue> {
    
    if key_bytes.len() != 32 || iv_bytes.len() != 16 {
        return Err(JsValue::from_str("キーまたはIVのサイズが不正です。"));
    }

    // AES-256-CBC 復号器の初期化
    let decryptor = Decryptor::<Aes256>::new_from_slices(key_bytes, iv_bytes)
        .map_err(|e| JsValue::from_str(&format!("復号器初期化失敗: {:?}", e)))?;

    let mut buffer = encrypted_data.to_vec();

    // 復号処理を実行 (Wasm内のネイティブ速度で処理)
    let decrypted_slice = decryptor.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("復号処理失敗: {:?}", e)))?;

    log("[WNSV RUST] チャンクの復号に成功しました。");
    Ok(decrypted_slice.to_vec())
}

// 将来的にSteam通信プロトコルのパケット処理を行う関数群をここに追加していきます
#[wasm_bindgen]
pub fn parse_steam_manifest(manifest_bytes: &[u8]) -> Result<JsValue, JsValue> {
    // Steamから取得したマニフェストファイル（ファイルリスト）の解析を行います。
    Ok(JsValue::from_str("マニフェスト解析完了（デモ）"))
}
