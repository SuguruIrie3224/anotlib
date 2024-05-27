# .anot ファイル仕様

## 概要
.anot ファイルは、画像、バウンディングボックス、およびコメントを含むアノテーションデータを格納するためのJSONベースのファイルフォーマットです。この仕様書では、.anot ファイルのデータ構造と、Rustライブラリを使用してこれらのデータを読み書きする方法について説明します。

## データ構造

### アノテーション全体
`Annotations`オブジェクトは、複数の`Annotation`オブジェクトを含むリストです。

```json
{
  "annotations": [
    {
      "image": {
        "filename": "example_image.jpg",
        "size": {
          "width": 1024,
          "height": 768
        },
        "base64": "base64-encoded-string-here"
      },
      "bounding_boxes": [
        {
          "label": "object1",
          "coordinates": {
            "x": 100,
            "y": 150,
            "width": 200,
            "height": 100
          },
          "comment": "This is a comment for object1."
        },
        {
          "label": "object2",
          "coordinates": {
            "x": 300,
            "y": 350,
            "width": 150,
            "height": 200
          },
          "comment": "This is a comment for object2."
        }
      ]
    },
    {
      "image": {
        "filename": "another_example_image.png",
        "size": {
          "width": 800,
          "height": 600
        },
        "base64": "base64-encoded-string-here"
      },
      "bounding_boxes": [
        {
          "label": "object3",
          "coordinates": {
            "x": 50,
            "y": 75,
            "width": 100,
            "height": 150
          },
          "comment": "This is a comment for object3."
        }
      ]
    }
  ]
}
```

### フィールド詳細

#### `Annotations`
- **annotations**: アノテーションのリスト。

#### `Annotation`
- **image**: 画像に関する情報。
    - **filename**: 画像ファイル名（文字列）。
    - **size**: 画像のサイズ（`ImageSize`オブジェクト）。
        - **width**: 画像の幅（整数）。
        - **height**: 画像の高さ（整数）。
    - **base64**: 画像データをBase64エンコードした文字列。

- **bounding_boxes**: バウンディングボックスのリスト（`BoundingBox`オブジェクト）。

#### `BoundingBox`
- **label**: バウンディングボックスに対応するオブジェクトのラベル（文字列）。
- **coordinates**: バウンディングボックスの座標（`Coordinates`オブジェクト）。
    - **x**: バウンディングボックスの左上のx座標（整数）。
    - **y**: バウンディングボックスの左上のy座標（整数）。
    - **width**: バウンディングボックスの幅（整数）。
    - **height**: バウンディングボックスの高さ（整数）。
- **comment**: バウンディングボックスに対するコメント（文字列）。

## RustライブラリのAPI

### 関数一覧

#### `from_file`
- **説明**: 指定されたファイルパスからアノテーションファイルを読み込み、`Annotations`構造体を返す。
- **引数**: `path: &str` - 読み込むファイルのパス。
- **戻り値**: `Result<Annotations, io::Error>`

#### `to_file`
- **説明**: 指定されたファイルパスにアノテーションデータをJSON形式で書き込む。
- **引数**: `path: &str` - 書き込むファイルのパス。
- **戻り値**: `Result<(), io::Error>`

#### `add_annotation`
- **説明**: 新しいアノテーションを追加する。
- **引数**: `annotation: Annotation`
- **戻り値**: `None`

#### `get_annotations`
- **説明**: すべてのアノテーションを返す。
- **引数**: なし
- **戻り値**: `&Vec<Annotation>`

## テスト項目

### 1. ファイルの読み込みテスト
- **テスト内容**: 存在するアノテーションファイルを正しく読み込めるか確認する。
- **期待結果**: ファイルから正しくデータが読み込まれる。

### 2. ファイルへの書き込みテスト
- **テスト内容**: データをファイルに正しく書き込めるか確認する。
- **期待結果**: データがファイルに正しく書き込まれる。

### 3. アノテーションの追加テスト
- **テスト内容**: 新しいアノテーションを正しく追加できるか確認する。
- **期待結果**: 新しいアノテーションが追加され、リストに存在する。

### 4. アノテーションの取得テスト
- **テスト内容**: すべてのアノテーションを正しく取得できるか確認する。
- **期待結果**: 追加されたすべてのアノテーションが正しく取得される。
