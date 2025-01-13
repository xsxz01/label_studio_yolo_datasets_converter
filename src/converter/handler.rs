pub fn deal_path(path: &str, output: &str) -> bool {
    // 检测是否存在所需要的文件夹
    // 要求文件夹目录存在文件夹 images 和 labels，以及文件 classes.txt 和 notes.json

    // 检测文件夹是否存在
    if !std::path::Path::new(path).exists() {
        println!("文件夹不存在");
        return false;
    }
    // 检测文件夹是否存在 images 和 labels 文件夹
    if !std::path::Path::new(path).join("images").exists() {
        println!("文件夹不存在 images");
        return false;
    }
    if !std::path::Path::new(path).join("labels").exists() {
        println!("文件夹不存在 labels");
        return false;
    }
    // 检测文件夹是否存在 classes.txt 和 notes.json 文件
    if !std::path::Path::new(path).join("classes.txt").exists() {
        println!("文件夹不存在 classes.txt");
        return false;
    }
    if !std::path::Path::new(path).join("notes.json").exists() {
        println!("文件夹不存在 notes.json");
        return false;
    }
    // 遍历文件夹，获取所有文件
    let mut images = Vec::new();
    let mut labels = Vec::new();
    let mut classes = Vec::new();
    // path/images/* 文件名存入images
    for entry in std::fs::read_dir(std::path::Path::new(path).join("images")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            images.push(path);
        }
    }
    // path/labels/* 文件名存入labels
    for entry in std::fs::read_dir(std::path::Path::new(path).join("labels")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            labels.push(path);
        }
    }
    // classes.txt 中每一行一个，存入classes
    let binding = std::fs::read_to_string(std::path::Path::new(path).join("classes.txt")).unwrap();
    for line in binding.lines() {
        classes.push(line);
    }
    // 获取images长度
    let images_len = images.len();
    // images拆分开，一个数组是train，一个数组是val，比例是长度的80%和20%
    let mut train_images = Vec::new();
    let mut val_images = Vec::new();
    for i in 0..images_len {
        if i < images_len * 8 / 10 {
            train_images.push(images[i].clone());
        } else {
            val_images.push(images[i].clone());
        }
    }
    // 获取labels长度
    let labels_len = labels.len();
    // labels拆分开，一个数组是train，一个数组是val，比例是长度的80%和20%
    let mut train_labels = Vec::new();
    let mut val_labels = Vec::new();
    for i in 0..labels_len {
        if i < labels_len * 8 / 10 {
            train_labels.push(labels[i].clone());
        } else {
            val_labels.push(labels[i].clone());
        }
    }
    // 组装data.yaml
    let mut data_yaml = String::new();
    data_yaml.push_str("path: ");
    data_yaml.push_str("./\n");
    data_yaml.push_str("train: ");
    data_yaml.push_str("images/train\n");
    data_yaml.push_str("val: ");
    data_yaml.push_str("images/val\n");
    data_yaml.push_str("nc: ");
    data_yaml.push_str(&classes.len().to_string());
    data_yaml.push_str("\n");
    data_yaml.push_str("names: ");
    // 写入例子如下 ['Abrasions','Animal scratches and bites','Crush Injury','Cuts','Impact injury','contusion']
    data_yaml.push_str("[");
    for i in 0..classes.len() {
        data_yaml.push_str("'");
        data_yaml.push_str(&classes[i]);
        data_yaml.push_str("'");
        if i != classes.len() - 1 {
            data_yaml.push_str(",");
        }
    }
    data_yaml.push_str("]");
    // 检测output文件夹是否存在，如果不存在则创建
    if !std::path::Path::new(output).exists() {
        std::fs::create_dir(std::path::Path::new(output)).unwrap();
    } else {
        // 如果存在则清空output文件夹内的文件
        std::fs::remove_dir_all(std::path::Path::new(output)).unwrap();
        std::fs::create_dir(std::path::Path::new(output)).unwrap();
    }
    // 写入data.yaml
    dbg!(std::path::Path::new(output).join("data.yaml"));
    std::fs::write(std::path::Path::new(output).join("data.yaml"), data_yaml).unwrap();
    // 写出ouput/datasets/images/train
    std::fs::create_dir_all(
        std::path::Path::new(output)
            .join("datasets")
            .join("images")
            .join("train"),
    )
    .unwrap();
    // 写出ouput/datasets/images/val
    std::fs::create_dir_all(
        std::path::Path::new(output)
            .join("datasets")
            .join("images")
            .join("val"),
    )
    .unwrap();
    // 写出ouput/datasets/labels/train
    std::fs::create_dir_all(
        std::path::Path::new(output)
            .join("datasets")
            .join("labels")
            .join("train"),
    )
    .unwrap();
    // 写出ouput/datasets/labels/val
    std::fs::create_dir_all(
        std::path::Path::new(output)
            .join("datasets")
            .join("labels")
            .join("val"),
    )
    .unwrap();
    // 复制images/train/* 到 output/datasets/images/train
    for i in 0..train_images.len() {
        std::fs::copy(
            train_images[i].clone(),
            std::path::Path::new(output)
                .join("datasets")
                .join("images")
                .join("train")
                .join(train_images[i].file_name().unwrap()),
        )
        .unwrap();
    }
    // 复制images/val/* 到 output/datasets/images/val
    for i in 0..val_images.len() {
        std::fs::copy(
            val_images[i].clone(),
            std::path::Path::new(output)
                .join("datasets")
                .join("images")
                .join("val")
                .join(val_images[i].file_name().unwrap()),
        )
        .unwrap();
    }
    // 复制labels/train/* 到 output/datasets/labels/train
    for i in 0..train_labels.len() {
        std::fs::copy(
            train_labels[i].clone(),
            std::path::Path::new(output)
                .join("datasets")
                .join("labels")
                .join("train")
                .join(train_labels[i].file_name().unwrap()),
        )
        .unwrap();
    }
    // 复制labels/val/* 到 output/datasets/labels/val
    for i in 0..val_labels.len() {
        std::fs::copy(
            val_labels[i].clone(),
            std::path::Path::new(output)
                .join("datasets")
                .join("labels")
                .join("val")
                .join(val_labels[i].file_name().unwrap()),
        )
        .unwrap();
    }
    return true;
}

pub fn deal_zip(path: &str, output: &str) -> bool {
    // 首先创建一个临时文件夹
    let temp_path = std::env::temp_dir().join("label_studio_yolo_datasets_converter");
    // 检测文件夹是否存在，如果不存在则创建
    if!std::path::Path::new(&temp_path).exists() {
        std::fs::create_dir(&temp_path).unwrap();
    } else {
        // 如果存在则清空文件夹内的文件
        std::fs::remove_dir_all(&temp_path).unwrap();
        std::fs::create_dir(&temp_path).unwrap();
    }
    // 解压文件到临时文件夹
    let mut zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
    zip.extract(&temp_path).unwrap();
    // 处理文件夹
    deal_path(&temp_path.to_str().unwrap(), output);
    return true;
}