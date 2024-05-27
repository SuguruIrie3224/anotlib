# anot_lib

anot_libは、画像アノテーションデータを格納および操作するためのRustライブラリです。画像、バウンディングボックス、コメントを含むアノテーションファイルをJSON形式で読み書きする機能を提供します。

## 特徴

- 画像アノテーションデータの読み書き
- 複数のバウンディングボックスの管理
- JSONベースのデータフォーマット

## インストール

このライブラリを使用するには、Gitリポジトリから依存関係を追加します。

### 1. Gitリポジトリから依存関係を追加

1. ライブラリをホスティングしているGitリポジトリを確認します。

2. 使用するプロジェクトの `Cargo.toml` ファイルに以下の依存関係を追加します。

```toml
[dependencies]
anot_lib = { git = "https://github.com/SuguruIrie3224/anotlib.git" }
```

### 2. 使用例

以下に、このライブラリを使用するプロジェクトの例を示します。

1. 新しいプロジェクトを作成します。

```sh
cargo new example_project
cd example_project
```

2. `Cargo.toml` にライブラリへの依存関係を追加します。

```toml
[dependencies]
anot_lib = { git = "https://github.com/yourusername/anot_lib.git" }
```

3. `src/main.rs` ファイルを編集してライブラリを使用します。

```rust
use anot_lib::{Annotations, Annotation, Image, ImageSize, BoundingBox, Coordinates};
use std::fs;

fn main() {
    // 新しいアノテーションを作成
    let annotation = Annotation {
        image: Image {
            filename: "example_image.jpg".to_string(),
            size: ImageSize { width: 1024, height: 768 },
            base64: "base64-encoded-string-here".to_string(),
        },
        bounding_boxes: vec![
            BoundingBox {
                label: "object1".to_string(),
                coordinates: Coordinates { x: 100, y: 150, width: 200, height: 100 },
                comment: "This is a comment for object1.".to_string(),
            },
        ],
    };

    // アノテーションリストを作成して追加
    let mut annotations = Annotations { annotations: vec![] };
    annotations.add_annotation(annotation);

    // アノテーションをファイルに保存
    let path = "annotations.anot";
    annotations.to_file(path).expect("Failed to write to file");

    // アノテーションをファイルから読み込み
    let loaded_annotations = Annotations::from_file(path).expect("Failed to read from file");

    // 読み込んだアノテーションを表示
    println!("{:?}", loaded_annotations);

    // テスト用ファイルを削除
    fs::remove_file(path).expect("Failed to delete test file");
}
```

## API

### from_file
指定されたファイルパスからアノテーションファイルを読み込み、`Annotations`構造体を返します。
```rust
fn from_file(path: &str) -> Result<Annotations, io::Error>
```

### to_file
指定されたファイルパスにアノテーションデータをJSON形式で書き込みます。
```rust
fn to_file(&self, path: &str) -> Result<(), io::Error>
```

### add_annotation
新しいアノテーションを追加します。
```rust
fn add_annotation(&mut self, annotation: Annotation)
```

### get_annotations
すべてのアノテーションを返します。
```rust
fn get_annotations(&self) -> &Vec<Annotation>
```

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細については、`LICENSE`ファイルを参照してください。
