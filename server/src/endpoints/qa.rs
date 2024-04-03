use axum::response::Html;

const QA_TEMPLATE: &str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Hello</title>
</head>

<body>
    <h1>Welcome to moyu server emulator</h1>
</body>
</html>
"#;
pub async fn qa_html() -> Html<&'static str> {
    Html(QA_TEMPLATE)
}
