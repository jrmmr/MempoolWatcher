use serde_json::Value;

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Trx {
    pub body: Body,
    pub auth_info: AuthInfo,
    pub signatures: Vec<String>,
}

impl Trx {
    pub fn get_messages(&self) -> Vec<Value> {
        self.body.messages.clone()
    }
}

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Body {
    pub messages: Vec<Value>,
    pub memo: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthInfo {
    pub fee: Value,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MsgExecuteContract {
    pub sender: String,
    pub contract: String,
    pub coins: Value,
    pub execute_msg: Value,
}

#[test]
fn test_sederialize_execute_contract() {
    let raw_trx = r#"{"body":{"messages":[{"@type":"/terra.wasm.v1beta1.MsgExecuteContract","sender":"terra179gxjhyuly8rvlkyn096g3ewtgvlnyksxxwqzq","contract":"terra16t7dpwwgx9n3lq6l6te3753lsjqwhxwpday9zx","execute_msg":{"execute_swap_operations":{"max_spread":"0.15","minimum_receive":"36837897","offer_amount":"4159915000","operations":[{"astro_swap":{"ask_asset_info":{"native_token":{"denom":"uluna"}},"offer_asset_info":{"native_token":{"denom":"uusd"}}}},{"astro_swap":{"ask_asset_info":{"token":{"contract_addr":"terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp"}},"offer_asset_info":{"native_token":{"denom":"uluna"}}}}]}},"coins":[{"denom":"uusd","amount":"4159915000"}]}],"memo":"","timeout_height":"0","extension_options":[],"non_critical_extension_options":[]},"auth_info":{"signer_infos":[{"public_key":{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AjIyAnafdL+R3vgdIWm+Ck5pYptDClLwKO1JqsB7ICl/"},"mode_info":{"single":{"mode":"SIGN_MODE_LEGACY_AMINO_JSON"}},"sequence":"556"}],"fee":{"amount":[{"denom":"uusd","amount":"320013"}],"gas_limit":"2133416","payer":"","granter":""}},"signatures":["Oo96QiKl9JybZCJ7ceaVB4MGXq+0fqmk2dZrE0qopYFsQZ6V/QPpLJSZCGgXrg4FBqaOX9mX10a8GYPcEgeKAQ=="]}"#;
    let trx = serde_json::from_str::<Trx>(&raw_trx.to_string()).unwrap();
    println!("{:?}", trx);
}
