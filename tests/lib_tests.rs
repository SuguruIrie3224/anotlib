use anotlib::{Annotation, Annotations, BoundingBox, Coordinates, Image, ImageSize};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_annotations_read_write() {
        let annotation = Annotations {
            annotations: vec![
                Annotation {
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
                        BoundingBox {
                            label: "object2".to_string(),
                            coordinates: Coordinates { x: 300, y: 350, width: 150, height: 200 },
                            comment: "This is a comment for object2.".to_string(),
                        },
                    ],
                },
            ],
        };

        let path = "test.anot";

        // ファイルに書き込む
        annotation.to_file(path).expect("Failed to write to file");

        // ファイルから読み込む
        let loaded_annotation = Annotations::from_file(path).expect("Failed to read from file");

        // 元のデータと読み込んだデータが一致するか確認
        assert_eq!(annotation.annotations.len(), loaded_annotation.annotations.len());
        assert_eq!(annotation.annotations[0].image.filename, loaded_annotation.annotations[0].image.filename);

        // テスト用ファイルを削除
        fs::remove_file(path).expect("Failed to delete test file");
    }

    #[test]
    fn test_add_annotation() {
        let mut annotations = Annotations { annotations: vec![] };

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

        annotations.add_annotation(annotation);

        assert_eq!(annotations.annotations.len(), 1);
        assert_eq!(annotations.annotations[0].image.filename, "example_image.jpg");
    }

    #[test]
    fn test_get_annotations() {
        let mut annotations = Annotations { annotations: vec![] };

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

        annotations.add_annotation(annotation);

        let all_annotations = annotations.get_annotations();
        assert_eq!(all_annotations.len(), 1);
        assert_eq!(all_annotations[0].image.filename, "example_image.jpg");
    }
}
