# sora-atlas

[English](README.md) | [简体中文](README.zh-CN.md)

macOS 视觉素材管理软件 **Atlas** 的本地 MCP 客户端强类型 Rust SDK。

专为 **Rust 2024 Edition**（Cargo 1.99.0+）打造，基于 `tokio` 与 `reqwest` 实现，零 async-trait 宏引入，零运行时额外开销。

---

## 核心特性

- **100% 覆盖官方工具**：完整实现官方 MCP Schema 中的全部 78 个工具并提供强类型保护。
- **原生 Rust 风格 API**：统一使用 Rust 惯用的 `snake_case` 方法名，与协议工具名保持一致（如 `client.search_items(&params).await`）。
- **零宏对象安全设计**：无 `async-trait` 过程宏开销，使用通用泛型结构 `AtlasClient<T: AtlasTransport = HttpAtlasTransport>`。
- **多图库 Token 自动切换**：支持自动读取图库目录下的 `.mcp-token`、从 `atlas status` 探测当前图库路径、并支持按图库路径配置独立密钥。
- **断线自愈与静默唤醒**：当本地 Atlas 软件未启动时，通过 `open -g -a Atlas` 静默拉起应用并轮询等待服务可用；当遭遇 `HTTP 401` 时自动清空缓存重新探测 Token 并重试一次。
- **协议参数自动清洗与补齐**：自动处理 MCP 协议的特殊参数要求（例如未设定的过滤字段自动补齐空字符串「」或数字「0」）。
- **内置脱机 Mock 传输层**：自带线程安全的 `MockAtlasTransport`，方便在不启动 Atlas 软件的情况下进行单元测试与参数断言。

---

## 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
sora-atlas = "0.1.0"
tokio = { version = "1.43", features = ["rt", "macros"] }
```

---

## 快速上手

### 1. 基础连接与检索

```rust
use sora_atlas::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 自动探测本地运行中图库的 Token 与端点（默认 http://127.0.0.1:50473/mcp）
    let client = AtlasClient::new();

    // 检查服务连通性
    let is_live = client.ping().await?;
    println!("Atlas MCP 服务就绪状态: {is_live}");

    // 获取当前打开图库的信息
    let info = client.library_info(&LibraryInfoParams::default()).await?;
    println!("已连接图库: {}，共有素材 {} 个", info.name, info.unique_items);

    // 检索素材
    let search_res = client
        .search_items(&SearchItemsParams {
            query: Some("设计规范".to_string()),
            kind: Some(ItemKind::Image),
            limit: Some(10),
            ..Default::default()
        })
        .await?;

    for item in search_res.items {
        println!("检索到素材 [{}]: {}", item.id, item.name);
    }

    Ok(())
}
```

### 2. 多图库 Token 配置与切换

每个 macOS 上的 `.atlaslibrary` 图库包在 `<图库路径>/.mcp-token` 都有自己独立的授权令牌。你可以在初始化时指定各个图库的 Token，也可以完全依赖自动发现：

```rust
use sora_atlas::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let mut library_tokens = HashMap::new();
    library_tokens.insert(
        "/Users/username/Pictures/工作.atlaslibrary".to_string(),
        "token_for_work".to_string(),
    );
    library_tokens.insert(
        "/Users/username/Pictures/个人.atlaslibrary".to_string(),
        "token_for_personal".to_string(),
    );

    let client = AtlasClient::with_options(HttpTransportOptions {
        token: std::env::var("ATLAS_MCP_TOKEN").ok(),
        library_tokens,
        auto_discover_token: true,
        auto_launch: true,
        ..Default::default()
    });

    // 切换当前激活的图库
    client
        .switch_library(&SwitchLibraryParams {
            path: "/Users/username/Pictures/工作.atlaslibrary".to_string(),
        })
        .await?;

    Ok(())
}
```

---

## 模块划分与支持的方法

`AtlasClient` 提供了 78 个强类型异步方法，分别组织在四个主要模块：

### 检索与查看模块 (`src/query.rs`)

- `search_items(&SearchItemsParams)`：元数据与全文检索
- `search_by_description(&SearchByDescriptionParams)`：自然语言语义向量搜索
- `find_similar(&FindSimilarParams)`：视觉相似素材匹配
- `get_item(&GetItemParams)`：读取素材完整详情
- `view_items(&ViewItemsParams)`：获取缩略图 Base64 画面
- `get_file(&GetFileParams)`：读取本地文件路径或原图二进制
- `store_embeddings(&StoreEmbeddingsParams)`：批量存入特征向量
- `query_embeddings(&QueryEmbeddingsParams)`：特征向量近邻检索
- `run_smart_folder(&RunSmartFolderParams)`：运行智能文件夹筛选
- `get_board(&GetBoardParams)`：获取画布集合排版与分组信息
- `get_current_view(&GetCurrentViewParams)`：读取软件当前窗口展示状态

### 素材录入与编辑模块 (`src/media.rs`)

- `import_files(&ImportFilesParams)`：从本地磁盘绝对路径导入文件
- `add_link(&AddLinkParams)`：添加网页链接卡片
- `edit_link(&EditLinkParams)`：编辑链接标题与描述
- `add_color(&AddColorParams)`：添加色卡
- `edit_color(&EditColorParams)`：修改色卡色值
- `save_media_from_url(&SaveMediaFromUrlParams)`：抓取网络媒体资源
- `edit_image(&EditImageParams)`：无损旋转与剪裁图片
- `refresh_thumbnail(&RefreshThumbnailParams)`：重新生成缩略图
- `set_liked(&SetLikedParams)`：点赞或取消点赞素材
- `set_metadata(&SetMetadataParams)`：修改备注、作者、拍摄地等
- `trash_items(&TrashItemsParams)`：移入回收站
- `restore_items(&RestoreItemsParams)`：从回收站恢复
- `delete_permanently(&DeletePermanentlyParams)`：彻底永久删除回收站素材
- `empty_trash(&EmptyTrashParams)`：清空回收站

### 集合、标签与图库管理模块 (`src/organize.rs`)

- **集合**：`list_collections()`、`create_collection(&CreateCollectionParams)`、`rename_collection(&RenameCollectionParams)`、`archive_collection(&ArchiveCollectionParams)`、`delete_collection(&DeleteCollectionParams)`、`set_collection_order(&SetCollectionOrderParams)`、`add_to_collection(&AddToCollectionParams)`、`remove_from_collection(&RemoveFromCollectionParams)`
- **文件夹**：`list_collection_folders()`、`create_collection_folder(&CreateCollectionFolderParams)`、`move_collection_folder(&MoveCollectionFolderParams)`、`rename_collection_folder(&RenameCollectionFolderParams)`、`delete_collection_folder(&DeleteCollectionFolderParams)`、`move_collection_to_folder(&MoveCollectionToFolderParams)`
- **标签与标签组**：`list_tags()`、`create_tag(&CreateTagParams)`、`rename_tag(&RenameTagParams)`、`delete_tag(&DeleteTagParams)`、`add_tags(&AddTagsParams)`、`remove_tags(&RemoveTagsParams)`、`set_tags(&SetTagsParams)`、`move_tags_to_folder(&MoveTagsToFolderParams)`、`create_tag_folder(&CreateTagFolderParams)`、`rename_tag_folder(&RenameTagFolderParams)`、`delete_tag_folder(&DeleteTagFolderParams)`
- **智能文件夹**：`list_smart_folders()`、`create_smart_folder(&CreateSmartFolderParams)`、`update_smart_folder(&UpdateSmartFolderParams)`、`delete_smart_folder(&DeleteSmartFolderParams)`
- **画布自由排版**：`arrange_board(&ArrangeBoardParams)`、`move_to_group(&MoveToGroupParams)`、`move_group(&MoveGroupParams)`、`reset_board_layout(&ResetBoardLayoutParams)`、`undo_board_layout(&UndoBoardLayoutParams)`
- **磁盘连接目录**：`list_connected_folders()`、`connect_folder(&ConnectFolderParams)`、`disconnect_folder(&DisconnectFolderParams)`、`rescan_folder(&RescanFolderParams)`
- **图库管理**：`library_info(&LibraryInfoParams)`、`list_libraries()`、`create_library(&CreateLibraryParams)`、`switch_library(&SwitchLibraryParams)`

### 软件界面与设置项模块 (`src/ui.rs`)

- `show_in_app(&ShowInAppParams)`：在软件窗口中跳转到指定页面或视图
- `show_items(&ShowItemsParams)`：在软件窗口中呈现指定的素材列表
- `open_item(&OpenItemParams)`：在软件窗口中打开素材详情
- `select_items(&SelectItemsParams)`：在软件窗口中框选选中素材
- `set_view_mode(&SetViewModeParams)`：切换显示模式（网格、无限瀑布流、画布）
- `show_in_finder(&ShowInFinderParams)`：在访达 Finder 中定位文件
- `get_settings(&GetSettingsParams)`：读取系统设置项
- `set_setting(&SetSettingParams)`：修改系统设置项
- `open_settings(&OpenSettingsParams)`：打开设置窗口
- `app_info()`：获取软件版本与系统环境信息
- `run_maintenance(&RunMaintenanceParams)`：运行图库去重或缩略图修复维护任务

---

## 自动化测试体系

### 脱机单元测试

无需启动 Atlas 软件，直接运行纯本地内存 Mock 单元测试集：

```bash
cargo test --test unit
```

### 真实端到端集成测试（E2E）

请确保本地 Atlas 客户端正在运行（端点 `http://127.0.0.1:50473/mcp`），随后执行：

```bash
cargo test --test e2e
```

E2E 测试严格遵守只读与安全测试规范：仅动态生成一个极简 1x1 像素的临时文件（`/tmp/atlas-rs-e2e-test.png`），完成导入校验与元数据修改测试后，在清理步骤中立即移入回收站并清理临时文件。

---

## 开源协议

MIT
