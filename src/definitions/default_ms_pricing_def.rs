pub const SBM_PRICING_DEF_JSON_STRING: &str = 
"{
  \"subscriptions\": [
    {
      \"code\": \"standard\",
      \"name\": \"Standard\",
      \"regex_pattern\": \"standard\",
      \"price\": 7.99,
      \"currency\": \"USD\"
    },
    {
      \"code\": \"pro\",
      \"name\": \"Pro\",
      \"regex_pattern\": \"pro\",
      \"price\": 27.99,
      \"currency\": \"USD\"
    }
  ],
  \"one_times\": [
    {
      \"code\": \"pack2k\",
      \"name\": \"2000 Labels\",
      \"regex_pattern\": \"2000\",
      \"price\": 11.99,
      \"currency\": \"USD\"
    },
    {
      \"code\": \"pack5k\",
      \"name\": \"5000 Labels\",
      \"regex_pattern\": \"5000\",
      \"price\": 22.99,
      \"currency\": \"USD\"
    },
    {
      \"code\": \"pack15k\",
      \"name\": \"15000 Labels\",
      \"regex_pattern\": \"15000\",
      \"price\": 44.99,
      \"currency\": \"USD\"
    }
  ]
}";